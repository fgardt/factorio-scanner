use std::{
    cell::RefCell,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use zip::ZipArchive;

use crate::mod_info::ModInfo;

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
    pub path: PathBuf,

    internal: ModType,
}

impl Mod {
    pub fn load<P: AsRef<Path>>(factorio_dir: P, name: &str) -> Result<Self> {
        Self::load_custom(&factorio_dir, factorio_dir.as_ref().join("mods"), name)
    }

    pub fn load_custom<FP: AsRef<Path>, MP: AsRef<Path>>(
        factorio_dir: FP,
        mod_dir: MP,
        name: &str,
    ) -> Result<Self> {
        let factorio_dir = factorio_dir.as_ref();
        let path = if Self::wube_mods().contains(&name) {
            factorio_dir.join("data")
        } else {
            mod_dir.as_ref().to_owned()
        }
        .join(name);

        let internal = ModType::load(&path)?;

        // the special core "mod" has no version field -> grab it from base instead
        let info = if name == "core" {
            let internal_base = ModType::load(factorio_dir.join("data/base"))?;
            let info_file = internal_base.get_file("info.json")?;
            let mut info = serde_json::from_slice::<ModInfo>(&info_file)
                .map_err(|err| ModError::InvalidInfoJson("base [to read core]".into(), err))?;
            info.name = "core".to_owned();
            info.title = "Core Factorio data".to_owned();
            info
        } else {
            let info_file = internal.get_file("info.json")?;
            serde_json::from_slice::<ModInfo>(&info_file)
                .map_err(|err| ModError::InvalidInfoJson(name.into(), err))?
        };

        #[allow(clippy::unwrap_used)] // known good regex
        let name_extractor = regex::Regex::new(r"^(.+?)(?:_\d+\.\d+\.\d+(?:\.zip)?)?$").unwrap();

        let name = name_extractor
            .captures(name)
            .ok_or(ModError::InvalidFilename(name.into()))?
            .get(1)
            .map(|n| n.as_str().to_owned())
            .ok_or(ModError::InvalidFilename(name.into()))?;

        if name != info.name {
            return Err(ModError::NameMismatch {
                expected: name,
                actual: info.name,
            });
        }

        Ok(Self {
            info,
            path,
            internal,
        })
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
        internal_prefix: String,
        zip: RefCell<ZipArchive<File>>,
    },
}

impl ModType {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            return Err(ModError::PathDoesNotExist(path.to_owned()));
        }

        if path.is_dir() {
            Ok(Self::Folder {
                path: path.to_owned(),
            })
        } else if path.is_file() && path.extension().unwrap_or_default() == "zip" {
            let zip = ZipArchive::new(File::open(path)?)?;
            let internal_prefix = zip
                .file_names()
                .next()
                .ok_or(ModError::ZipEmpty(path.to_owned()))?
                .split('/')
                .next()
                .ok_or(ModError::UnknownInternalFolder(path.to_owned()))?
                .to_owned()
                + "/";

            Ok(Self::Zip {
                internal_prefix,
                zip: RefCell::new(zip),
            })
        } else {
            return Err(ModError::PathNotZipOrDir(path.to_owned()));
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
