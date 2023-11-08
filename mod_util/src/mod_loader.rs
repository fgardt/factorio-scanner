use std::{
    cell::RefCell,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use zip::ZipArchive;

use crate::mod_info::ModInfo;

#[derive(Debug)]
pub struct Mod {
    pub info: ModInfo,

    internal: ModType,
}

impl Mod {
    pub fn load(factorio_dir: &Path, name: &str) -> anyhow::Result<Self> {
        let path = if Self::wube_mods().contains(&name) {
            factorio_dir.join("data")
        } else {
            factorio_dir.join("mods")
        }
        .join(name);

        let internal = ModType::load(path)?;

        // the special core "mod" has no version field -> grab it from base instead
        let info = if name == "core" {
            let internal_base = ModType::load(factorio_dir.join("data/base"))?;
            let info_file = internal_base.get_file("info.json")?;
            let mut info = serde_json::from_slice::<ModInfo>(&info_file)?;
            info.name = "core".to_owned();
            info.title = "Core Factorio data".to_owned();
            info
        } else {
            let info_file = internal.get_file("info.json")?;
            serde_json::from_slice::<ModInfo>(&info_file)?
        };

        #[allow(clippy::unwrap_used)] // known good regex
        let name_extractor = regex::Regex::new(r"^(.+?)(?:_\d+\.\d+\.\d+(?:\.zip)?)?$").unwrap();

        let Some(extracted) = name_extractor.captures(&name) else {
            anyhow::bail!("mod filename does not match expected format: {name}");
        };
        let Some(name) = extracted.get(1).map(|n| n.as_str().to_owned()) else {
            anyhow::bail!("mod filename does not match expected format: {name}");
        };

        if name != info.name {
            anyhow::bail!(
                "Mod name does not match name in info.json: {name} != {}",
                info.name
            );
        }

        Ok(Self { info, internal })
    }

    pub fn get_file(&self, path: &str) -> anyhow::Result<Vec<u8>> {
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
    fn load(path: PathBuf) -> anyhow::Result<Self> {
        if !path.exists() {
            anyhow::bail!("Mod path does not exist: {path:?}");
        }

        if path.is_dir() {
            Ok(Self::Folder { path })
        } else if path.is_file() && path.extension().unwrap_or_default() == "zip" {
            let zip = ZipArchive::new(File::open(&path)?)?;
            let internal_prefix = zip
                .file_names()
                .next()
                .ok_or_else(|| anyhow::anyhow!("Mod zip is empty: {path:?}"))?
                .split('/')
                .next()
                .ok_or_else(|| anyhow::anyhow!("Could not get mod zips internal folder: {path:?}"))?
                .to_owned()
                + "/";

            Ok(Self::Zip {
                internal_prefix,
                zip: RefCell::new(zip),
            })
        } else {
            anyhow::bail!("Mod path is not a zip file or directory: {path:?}")
        }
    }

    fn get_file(&self, file: &str) -> anyhow::Result<Vec<u8>> {
        match self {
            Self::Folder { path } => {
                let path = path.join(file);
                if !path.exists() {
                    anyhow::bail!("Mod file does not exist: {path:?}");
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
