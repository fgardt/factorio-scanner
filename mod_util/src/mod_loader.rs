use std::{
    cell::RefCell,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use zip::ZipArchive;

use crate::mod_info::{ModInfo, Version};

#[derive(Debug, thiserror::Error)]
pub enum ModError {
    #[error("mod path does not exist: {0:?}")]
    PathDoesNotExist(PathBuf),

    #[error("mod path is not a zip file or directory: {0:?}")]
    PathNotZipOrDir(PathBuf),

    #[error("mod zip is empty: {0:?}")]
    ZipEmpty(PathBuf),

    #[error("could not get mod zips internal folder: {0:?}")]
    UnknownInternalFolder(PathBuf),

    #[error("unable to parse info.json of {0}: {1}")]
    InvalidInfoJson(String, serde_json::Error),

    #[error("mod filename does not match expected format: {0}")]
    InvalidFilename(String),

    #[error("mod name does not match name in info.json: {expected} != {actual}")]
    NameMismatch { expected: String, actual: String },

    #[error("mod version does not match version in info.json: {expected} != {actual}")]
    VersionMismatch { expected: Version, actual: Version },

    #[error("mod io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("mod zip error: {0}")]
    ZipError(#[from] zip::result::ZipError),

    #[error("mod mutable borrow error: {0}")]
    BorrowError(#[from] std::cell::BorrowMutError),
}

type Result<T> = std::result::Result<T, ModError>;

#[derive(Debug)]
pub struct Mod {
    pub info: ModInfo,

    internal: ModType,
}

impl Mod {
    pub fn load(factorio_dir: impl AsRef<Path>, name: &str, version: Version) -> Result<Self> {
        Self::load_custom(
            factorio_dir.as_ref().join("data"),
            factorio_dir.as_ref().join("mods"),
            name,
            version,
        )
    }

    pub fn load_custom(
        read_path: impl AsRef<Path>,
        mods_path: impl AsRef<Path>,
        name: &str,
        version: Version,
    ) -> Result<Self> {
        if Self::wube_mods().contains(&name) {
            return Self::load_wube(read_path, name);
        }

        let internal = ModType::load(&mods_path, name, version)?;

        let info_file = internal.get_file("info.json")?;
        let info = serde_json::from_slice::<ModInfo>(&info_file)
            .map_err(|err| ModError::InvalidInfoJson(name.into(), err))?;

        if info.version != version {
            return Err(ModError::VersionMismatch {
                expected: version,
                actual: info.version,
            });
        }

        #[allow(clippy::unwrap_used)] // known good regex
        let name_extractor = regex::Regex::new(r"^(.+?)(?:_\d+\.\d+\.\d+(?:\.zip)?)?$").unwrap();

        let name = name_extractor
            .captures(name)
            .ok_or_else(|| ModError::InvalidFilename(name.into()))?
            .get(1)
            .map(|n| n.as_str().to_owned())
            .ok_or_else(|| ModError::InvalidFilename(name.into()))?;

        if name != info.name {
            return Err(ModError::NameMismatch {
                expected: name,
                actual: info.name,
            });
        }

        Ok(Self { info, internal })
    }

    pub fn load_wube(read_path: impl AsRef<Path>, name: &str) -> Result<Self> {
        if !Self::wube_mods().contains(&name) {
            return Err(ModError::PathDoesNotExist(read_path.as_ref().join(name)));
        }

        let path = read_path.as_ref().join(name);

        if !path.exists() {
            return Err(ModError::PathDoesNotExist(path));
        }

        let internal = ModType::load_from_path(&path)?;

        // the special core "mod" has no version field -> grab it from base instead
        let info = if name == "core" {
            let internal_base = ModType::load_from_path(read_path.as_ref().join("base"))?;
            let info_file = internal_base.get_file("info.json")?;
            let mut info = serde_json::from_slice::<ModInfo>(&info_file)
                .map_err(|err| ModError::InvalidInfoJson("base [to read core]".into(), err))?;
            "core".clone_into(&mut info.name);
            "Core Factorio data".clone_into(&mut info.title);
            info
        } else {
            let info_file = internal.get_file("info.json")?;
            serde_json::from_slice::<ModInfo>(&info_file)
                .map_err(|err| ModError::InvalidInfoJson(name.into(), err))?
        };

        Ok(Self { info, internal })
    }

    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self> {
        let internal = ModType::load_from_path(&path)?;

        let info_file = internal.get_file("info.json")?;
        let info = serde_json::from_slice::<ModInfo>(&info_file)
            .map_err(|err| ModError::InvalidInfoJson(path.as_ref().display().to_string(), err))?;

        Ok(Self { info, internal })
    }

    pub fn path(&self) -> PathBuf {
        match self.internal {
            ModType::Folder { ref path } | ModType::Zip { ref path, .. } => path.clone(),
        }
    }

    pub fn get_file(&self, path: &str) -> Result<Vec<u8>> {
        self.internal.get_file(path)
    }

    #[must_use]
    pub const fn wube_mods() -> [&'static str; 5] {
        ["core", "base", "elevated-rails", "quality", "space-age"]
    }
}

#[derive(Debug)]
enum ModType {
    Folder {
        path: PathBuf,
    },
    Zip {
        path: PathBuf,
        internal_prefix: String,
        zip: RefCell<ZipArchive<File>>,
    },
}

impl ModType {
    fn load(path: impl AsRef<Path>, name: &str, version: Version) -> Result<Self> {
        let zip_path = path.as_ref().join(format!("{name}_{version}.zip"));
        let (path, is_zip) = if zip_path.exists() && zip_path.is_file() {
            (zip_path, true)
        } else {
            let folder_path = path.as_ref().join(name);

            if !folder_path.exists() {
                return Err(ModError::PathDoesNotExist(folder_path));
            }

            (folder_path, false)
        };

        if is_zip {
            let zip = ZipArchive::new(File::open(&path)?)?;
            let internal_prefix = get_zip_internal_folder(&path, &zip)?;

            Ok(Self::Zip {
                path,
                internal_prefix,
                zip: RefCell::new(zip),
            })
        } else if path.is_dir() {
            Ok(Self::Folder { path })
        } else {
            Err(ModError::PathNotZipOrDir(path))
        }
    }

    fn load_from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(ModError::PathDoesNotExist(path.into()));
        }

        if path.is_dir() {
            Ok(Self::Folder { path: path.into() })
        } else if path.is_file() && path.extension().is_some_and(|ext| ext == "zip") {
            let zip = ZipArchive::new(File::open(path)?)?;
            let internal_prefix = get_zip_internal_folder(path, &zip)?;

            Ok(Self::Zip {
                path: path.into(),
                internal_prefix,
                zip: RefCell::new(zip),
            })
        } else {
            Err(ModError::PathNotZipOrDir(path.into()))
        }
    }

    fn get_file(&self, file: &str) -> Result<Vec<u8>> {
        match self {
            Self::Folder { path } => {
                let path = path.join(file);
                if !path.exists() {
                    return Err(ModError::PathDoesNotExist(path));
                }

                Ok(std::fs::read(path)?)
            }
            Self::Zip {
                internal_prefix,
                zip,
                ..
            } => {
                let path = internal_prefix.clone() + file;
                let mut zip = zip.try_borrow_mut()?;
                let mut file = zip.by_name(&path)?;

                // if the vec allocates not enough it will just reallocate
                #[allow(clippy::cast_possible_truncation)]
                let mut bytes = Vec::with_capacity(file.size() as usize);

                file.read_to_end(&mut bytes)?;
                Ok(bytes)
            }
        }
    }
}

fn get_zip_internal_folder(path: impl AsRef<Path>, zip: &ZipArchive<File>) -> Result<String> {
    let res = zip
        .file_names()
        .next()
        .ok_or_else(|| ModError::ZipEmpty(path.as_ref().into()))?
        .split('/')
        .next()
        .ok_or_else(|| ModError::UnknownInternalFolder(path.as_ref().into()))?
        .to_owned()
        + "/";

    Ok(res)
}
