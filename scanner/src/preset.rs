use clap::builder::PossibleValue;
use strum::{EnumIter, VariantArray};

use mod_util::{
    mod_info::{DependencyVersion, Version},
    DependencyList,
};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, VariantArray)]
pub enum Preset {
    K2,
    SE,
    K2SE,
    IR3,
    PyAE,
    FF,
    FFK2,
    EI,
    EIK2,
    Nullius,
    SeaBlock,
    Ultracube,
}

macro_rules! mod_p {
    ( $( $name:literal $ver:literal:$ver2:literal:$ver3:literal ),* ) => {
        vec![
            $(
                ($name.to_owned(), DependencyVersion::HigherOrEqual(Version::new($ver, $ver2, $ver3))),
            )+
        ]
    };
}

impl Preset {
    #[must_use]
    pub fn used_mods(self) -> DependencyList {
        match self {
            Self::K2 => mod_p!["Krastorio2" 1:3:23],
            Self::SE => mod_p!["space-exploration" 0:6:130],
            Self::K2SE => mod_p![
                "Krastorio2" 1:3:23,
                "space-exploration" 0:6:130
            ],
            Self::IR3 => mod_p!["IndustrialRevolution3" 3:1:20],
            Self::PyAE => mod_p!["pyalternativeenergy" 1:2:25],
            Self::FF => mod_p!["FreightForwardingPack" 1:2:1],
            Self::FFK2 => mod_p![
                "FreightForwardingPack" 1:2:1,
                "Krastorio2" 1:3:23
            ],
            Self::EI => mod_p!["exotic-industries-modpack" 0:5:10],
            Self::EIK2 => mod_p![
                "exotic-industries-modpack" 0:5:10,
                "Krastorio2" 1:3:23
            ],
            Self::Nullius => mod_p!["nullius" 1:9:1],
            Self::SeaBlock => mod_p!["SeaBlockMetaPack" 1:1:4],
            Self::Ultracube => mod_p!["Ultracube" 0:5:4],
        }
        .iter()
        .cloned()
        .collect()
    }

    // TODO: used settings

    #[must_use]
    pub const fn known_prefix(self) -> Option<&'static str> {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::K2 => Some("kr-"),
            Self::SE => Some("se-"),
            Self::K2SE => None,
            Self::IR3 => None,
            Self::PyAE => Some("py-"),
            Self::FF => Some("ff-"),
            Self::FFK2 => None,
            Self::EI => Some("ei_"),
            Self::EIK2 => None,
            Self::Nullius => Some("nullius-"),
            Self::SeaBlock => None,
            Self::Ultracube => Some("cube-"),
        }
    }
}

impl ToString for Preset {
    fn to_string(&self) -> String {
        format!("{self:?}")
    }
}

impl std::str::FromStr for Preset {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "k2" => Ok(Self::K2),
            "se" => Ok(Self::SE),
            "k2se" | "k2+se" | "sek2" | "se+k2" => Ok(Self::K2SE),
            "ir3" => Ok(Self::IR3),
            "pyae" | "pyanodons" => Ok(Self::PyAE),
            "ff" | "freightforwarding" => Ok(Self::FF),
            "ffk2" => Ok(Self::FFK2),
            "ei" | "exoticindustries" => Ok(Self::EI),
            "eik2" | "ei+k2" | "k2ei" | "k2+ei" => Ok(Self::EIK2),
            "nullius" => Ok(Self::Nullius),
            "seablock" | "sb" => Ok(Self::SeaBlock),
            "ultracube" => Ok(Self::Ultracube),
            _ => Err(format!("unknown preset: {s}")),
        }
    }
}

impl clap::ValueEnum for Preset {
    fn value_variants<'a>() -> &'a [Self] {
        Self::VARIANTS
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Self::K2 => Some(PossibleValue::new("K2").alias("k2")),
            Self::SE => Some(PossibleValue::new("SE").alias("se")),
            Self::K2SE => Some(
                PossibleValue::new("K2SE")
                    .aliases(["k2se", "K2+SE", "k2+se", "SEK2", "sek2", "SE+K2", "se+k2"]),
            ),
            Self::IR3 => Some(PossibleValue::new("IR3").alias("ir3")),
            Self::PyAE => Some(PossibleValue::new("PyAE").aliases(["pyae", "pyanodons"])),
            Self::FF => Some(PossibleValue::new("FF").aliases(["ff", "FreightForwarding"])),
            Self::FFK2 => Some(PossibleValue::new("FFK2").aliases(["ffk2", "FF+K2", "ff+k2"])),
            Self::EI => Some(PossibleValue::new("EI").aliases(["ei", "exoticindustries"])),
            Self::EIK2 => Some(
                PossibleValue::new("EIK2")
                    .aliases(["eik2", "EI+K2", "ei+k2", "K2EI", "k2ei", "K2+EI", "k2+ei"]),
            ),
            Self::Nullius => Some(PossibleValue::new("Nullius").alias("nullius")),
            Self::SeaBlock => {
                Some(PossibleValue::new("SeaBlock").aliases(["seablock", "SB", "sb"]))
            }
            Self::Ultracube => Some(PossibleValue::new("Ultracube").alias("ultracube")),
        }
    }
}
