use std::{
    cell::RefCell,
    fs::File,
    io::{Read, Seek},
    path::{Path, PathBuf},
};

use zip::read::ZipFile;
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

    #[error("mod version does not match version in info.json: {name} {expected} != {actual}")]
    VersionMismatch {
        name: String,
        expected: Version,
        actual: Version,
    },

    #[error("mod not found: {0} v{1}")]
    ModNotFound(String, Version),

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
        let info = internal.get_info(name)?;

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

    // Returns a list of all files/subdirectories that live in `dir`.
    pub fn read_dir(&self, dir: &str) -> Result<Vec<ModEntry>> {
        self.internal.read_dir(dir)
    }

    #[must_use]
    pub const fn wube_mods() -> [&'static str; 5] {
        ["core", "base", "elevated-rails", "quality", "space-age"]
    }
}

/// An object which represents a file or folder inside of a Mod, like `fs::DirEntry`.
#[derive(Debug)]
pub struct ModEntry {
    path: PathBuf,
    is_file: bool,
    // Can add all sorts of fancy info as we like
}

impl ModEntry {
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
    pub fn is_file(&self) -> bool {
        self.is_file
    }
    pub fn is_dir(&self) -> bool {
        !self.is_file
    }
    // I don't think we have to worry about symlinks...
}

impl TryFrom<std::fs::DirEntry> for ModEntry {
    type Error = ModError;

    fn try_from(item: std::fs::DirEntry) -> Result<ModEntry> {
        let metadata = item.metadata()?;
        Ok(ModEntry {
            path: item.path(),
            is_file: metadata.is_file(),
        })
    }
}

impl TryFrom<&ZipFile<'_, File>> for ModEntry {
    type Error = ModError;

    fn try_from(item: &ZipFile<'_, File>) -> Result<ModEntry> {
        Ok(ModEntry {
            path: item.name().into(),
            is_file: item.is_file(),
        })
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
        // unversioned folder
        let target_path = path.as_ref().join(name);
        if target_path.exists() && target_path.is_dir() {
            let tmp = Self::Folder { path: target_path };

            if let Ok(info) = tmp.get_info(name) {
                verify_info(&info, name, version)?;
            }

            return Ok(tmp);
        }

        // versioned folder
        let target_path = path.as_ref().join(format!("{name}_{version}"));
        if target_path.exists() && target_path.is_dir() {
            let tmp = Self::Folder { path: target_path };

            if let Ok(info) = tmp.get_info(name) {
                verify_info(&info, name, version)?;
            }

            return Ok(tmp);
        }

        // zip
        let target_path = path.as_ref().join(format!("{name}_{version}.zip"));
        if target_path.exists() && target_path.is_file() {
            let zip = ZipArchive::new(File::open(&target_path)?)?;
            let internal_prefix = get_zip_internal_folder(&target_path, &zip)?;

            let tmp = Self::Zip {
                path: target_path,
                internal_prefix,
                zip: RefCell::new(zip),
            };

            if let Ok(info) = tmp.get_info(name) {
                verify_info(&info, name, version)?;
            }

            return Ok(tmp);
        }

        Err(ModError::ModNotFound(name.to_owned(), version))
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

    fn read_dir(&self, dir: &str) -> Result<Vec<ModEntry>> {
        match self {
            Self::Folder { path } => {
                let path = path.join(dir);
                if !path.exists() {
                    return Err(ModError::PathDoesNotExist(path));
                }

                return std::fs::read_dir(&path)?
                    .filter_map(|x| x.ok())
                    .map(|x| ModEntry::try_from(x))
                    .collect();
            }
            Self::Zip {
                internal_prefix,
                zip,
                ..
            } => {
                let path = internal_prefix.clone() + dir;
                let mut zip = zip.try_borrow_mut()?;

                if let Err(_) = &zip.by_name(&path) {
                    return Err(ModError::PathDoesNotExist(path.into()));
                }

                // We need to copy all of the `&str`s to `String`s so we don't
                // keep the immutable ref to `zip` alive and trigger a double borrow
                let entries: Vec<String> = zip
                    .file_names()
                    .filter(|&x| x.starts_with(&path) && x != path)
                    .map(|x| x.into())
                    .collect();

                return entries
                    .iter()
                    .map(|x| ModEntry::try_from(&zip.by_name(x)?))
                    .collect();
            }
        }
    }
      
    fn get_info(&self, name: &str) -> Result<ModInfo> {
        let info_file = self.get_file("info.json")?;
        let info = serde_json::from_slice(&info_file)
            .map_err(|err| ModError::InvalidInfoJson(name.into(), err))?;

        Ok(info)
    }
}

pub fn get_zip_internal_folder<R: Read + Seek>(
    path: impl AsRef<Path>,
    zip: &ZipArchive<R>,
) -> Result<String> {
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

fn verify_info(info: &ModInfo, expected_name: &str, expected_version: Version) -> Result<()> {
    if info.name != expected_name {
        return Err(ModError::NameMismatch {
            expected: expected_name.to_owned(),
            actual: info.name.clone(),
        });
    }

    if info.version != expected_version {
        return Err(ModError::VersionMismatch {
            name: info.name.clone(),
            expected: expected_version,
            actual: info.version,
        });
    }

    Ok(())
}
