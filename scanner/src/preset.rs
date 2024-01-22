use clap::builder::PossibleValue;
use mod_util::{mod_info::Version, UsedVersions};
use strum::EnumIter;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Preset {
    K2,
    SE,
    K2SE,
    SeaBlock,
}

impl Preset {
    pub fn used_mods(self) -> UsedVersions {
        match self {
            Self::K2 => vec![("Krastorio2".to_owned(), Version::new(1, 3, 23))],
            Self::SE => vec![("space-exploration".to_owned(), Version::new(0, 6, 119))],
            Self::K2SE => vec![
                ("Krastorio2".to_owned(), Version::new(1, 3, 23)),
                ("space-exploration".to_owned(), Version::new(0, 6, 119)),
            ],
            Self::SeaBlock => vec![("SeaBlockMetaPack".to_owned(), Version::new(1, 1, 4))],
        }
        .iter()
        .cloned()
        .collect()
    }

    // TODO: used settings
}

impl ToString for Preset {
    fn to_string(&self) -> String {
        match self {
            Self::K2 => "K2",
            Self::SE => "SE",
            Self::K2SE => "K2SE",
            Self::SeaBlock => "SeaBlock",
        }
        .to_owned()
    }
}

impl std::str::FromStr for Preset {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "k2" => Ok(Self::K2),
            "se" => Ok(Self::SE),
            "k2se" | "k2+se" | "sek2" | "se+k2" => Ok(Self::K2SE),
            "seablock" | "sb" => Ok(Self::SeaBlock),
            _ => Err(format!("unknown preset: {s}")),
        }
    }
}

impl clap::ValueEnum for Preset {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::K2, Self::SE, Self::K2SE, Self::SeaBlock]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Self::K2 => Some(PossibleValue::new("K2").alias("k2")),
            Self::SE => Some(PossibleValue::new("SE").alias("se")),
            Self::K2SE => Some(
                PossibleValue::new("K2SE")
                    .aliases(["k2se", "K2+SE", "k2+se", "SEK2", "sek2", "SE+K2", "se+k2"]),
            ),
            Self::SeaBlock => {
                Some(PossibleValue::new("SeaBlock").aliases(["seablock", "SB", "sb"]))
            }
        }
    }
}
