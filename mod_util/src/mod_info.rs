use core::fmt;

use serde::{de::Error, Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ModInfo {
    pub name: String,
    pub version: String,
    pub title: String,
    pub author: String,

    pub contact: Option<String>,
    pub homepage: Option<String>,
    pub description: Option<String>,

    pub factorio_version: Option<String>,

    pub dependencies: Vec<Dependency>,
}

#[derive(Debug)]
pub struct Dependency {
    kind: DependencyType,
    name: String,
    version: DependencyVersion,
}

impl fmt::Display for Dependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = &self.kind;
        let name = &self.name;
        let version = &self.version;

        write!(f, "{prefix} {name}{version}")
    }
}

#[derive(Debug)]
enum DependencyType {
    Incompatible,
    Optional,
    HiddenOptional,
    Lazy,
    Required,
}

impl fmt::Display for DependencyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Incompatible => "!",
            Self::Optional => "?",
            Self::HiddenOptional => "(?)",
            Self::Lazy => "~",
            Self::Required => "",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug)]
enum DependencyVersion {
    Any,
    Lower(String),
    LowerOrEqual(String),
    Equal(String),
    HigherOrEqual(String),
    Higher(String),
}

impl fmt::Display for DependencyVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Any => String::new(),
            Self::Lower(version) => format!(" < {version}"),
            Self::LowerOrEqual(version) => format!(" <= {version}"),
            Self::Equal(version) => format!(" = {version}"),
            Self::HigherOrEqual(version) => format!(" >= {version}"),
            Self::Higher(version) => format!(" > {version}"),
        };

        write!(f, "{s}")
    }
}

impl Serialize for Dependency {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("{self}"))
    }
}

impl<'de> Deserialize<'de> for Dependency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[allow(clippy::unwrap_used)] // The regex is hardcoded and will always compile
        let extractor =
            regex::Regex::new(r"(?:(!|\?|\(\?\)|~)\s)?(\S+)(?:\s(>=?|<=?|=)\s(\d+\.\d+\.\d+))?")
                .unwrap();

        let dep = String::deserialize(deserializer)?;
        let Some(captures) = extractor.captures(&dep) else {
            return Err(Error::custom("Invalid dependency"));
        };

        let kind = captures
            .get(1)
            .map_or(DependencyType::Required, |kind| match kind.as_str() {
                "!" => DependencyType::Incompatible,
                "?" => DependencyType::Optional,
                "(?)" => DependencyType::HiddenOptional,
                "~" => DependencyType::Lazy,
                _ => unreachable!("Dependency kinds are checked by the regex"),
            });

        let Some(name) = captures.get(2).map(|n| n.as_str().to_owned()) else {
            return Err(Error::custom("Invalid dependency"));
        };

        let version =
            captures
                .get(3)
                .map(|c| c.as_str())
                .map_or(DependencyVersion::Any, |comparator| {
                    let version = captures.get(4).map(|v| v.as_str().to_owned());
                    match (comparator, version) {
                        ("<", Some(version)) => DependencyVersion::Lower(version),
                        ("<=", Some(version)) => DependencyVersion::LowerOrEqual(version),
                        ("=", Some(version)) => DependencyVersion::Equal(version),
                        (">=", Some(version)) => DependencyVersion::HigherOrEqual(version),
                        (">", Some(version)) => DependencyVersion::Higher(version),
                        _ => {
                            unreachable!("Dependency comparator & version are checked by the regex")
                        }
                    }
                });

        Ok(Self {
            kind,
            name,
            version,
        })
    }
}
