use core::fmt;
use std::str::FromStr;

use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer, Serialize,
};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModInfo {
    pub name: String,
    pub version: Version,
    pub title: String,
    pub author: String,

    pub contact: Option<String>,
    pub homepage: Option<String>,
    pub description: Option<String>,

    pub factorio_version: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Version {
    major: u16,
    minor: u16,
    patch: u16,
}

impl Version {
    #[must_use]
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let major = self.major.cmp(&other.major);
        let minor = self.minor.cmp(&other.minor);
        let patch = self.patch.cmp(&other.patch);

        if major != std::cmp::Ordering::Equal {
            return major;
        }
        if minor != std::cmp::Ordering::Equal {
            return minor;
        }
        if patch != std::cmp::Ordering::Equal {
            return patch;
        }

        std::cmp::Ordering::Equal
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let major = &self.major;
        let minor = &self.minor;
        let patch = &self.patch;

        write!(f, "{major}.{minor}.{patch}")
    }
}

impl FromStr for Version {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::try_from(value)
    }
}

impl TryFrom<&str> for Version {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split('.');

        let major = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("expected 3 parts"))?
            .parse()
            .map_err(anyhow::Error::new)?;
        let minor = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("expected 3 parts"))?
            .parse()
            .map_err(anyhow::Error::new)?;
        let patch = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("expected 3 parts"))?
            .parse()
            .map_err(anyhow::Error::new)?;

        Ok(Self {
            major,
            minor,
            patch,
        })
    }
}

impl TryFrom<String> for Version {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&String> for Version {
    type Error = anyhow::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl From<Version> for String {
    fn from(version: Version) -> Self {
        format!("{version}")
    }
}

impl From<&Version> for String {
    fn from(version: &Version) -> Self {
        format!("{version}")
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{self}"))
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(VersionVisitor)
    }
}

struct VersionVisitor;

impl<'de> Visitor<'de> for VersionVisitor {
    type Value = Version;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(
            "a version in the format \"major.minor.patch\" where each part is a u16 or a single packed u64",
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let mut parts = v.split('.');

        let major = parts
            .next()
            .ok_or_else(|| Error::custom("expected 3 parts"))?
            .parse()
            .map_err(Error::custom)?;
        let minor = parts.next().unwrap_or("0").parse().map_err(Error::custom)?; // minor will default to 0 if missing
        let patch = parts.next().unwrap_or("0").parse().map_err(Error::custom)?; // patch will default to 0 if missing

        Ok(Self::Value {
            major,
            minor,
            patch,
        })
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_str(&v)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum DependencyType {
    Incompatible,
    Optional,
    HiddenOptional,
    RequiredLazy,
    Required,
}

impl DependencyType {
    pub const fn is_required(&self) -> bool {
        matches!(self, Self::RequiredLazy | Self::Required)
    }

    pub const fn is_optional(&self) -> bool {
        matches!(self, Self::Optional | Self::HiddenOptional)
    }

    pub const fn is_incompatible(&self) -> bool {
        matches!(self, Self::Incompatible)
    }
}

impl fmt::Display for DependencyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Incompatible => "!",
            Self::Optional => "?",
            Self::HiddenOptional => "(?)",
            Self::RequiredLazy => "~",
            Self::Required => "",
        };
        write!(f, "{s}")
    }
}

impl Serialize for DependencyType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("{self}"))
    }
}

impl<'de> Deserialize<'de> for DependencyType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DependencyTypeVisitor)
    }
}

struct DependencyTypeVisitor;

impl<'de> Visitor<'de> for DependencyTypeVisitor {
    type Value = DependencyType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a dependency type: !, ?, (?), ~ or nothing")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match v {
            "!" => Ok(DependencyType::Incompatible),
            "?" => Ok(DependencyType::Optional),
            "(?)" => Ok(DependencyType::HiddenOptional),
            "~" => Ok(DependencyType::RequiredLazy),
            "" => Ok(DependencyType::Required),
            _ => Err(Error::custom("Invalid dependency type")),
        }
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_str(&v)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DependencyVersion {
    Any,
    Lower(Version),
    LowerOrEqual(Version),
    Exact(Version),
    HigherOrEqual(Version),
    Higher(Version),
}

impl DependencyVersion {
    #[must_use]
    pub fn allows(&self, version: Version) -> bool {
        match self {
            Self::Any => true,
            Self::Lower(v) => version < *v,
            Self::LowerOrEqual(v) => version <= *v,
            Self::Exact(v) => version == *v,
            Self::HigherOrEqual(v) => version >= *v,
            Self::Higher(v) => version > *v,
        }
    }

    #[must_use]
    pub const fn is_any(&self) -> bool {
        matches!(self, Self::Any)
    }

    #[must_use]
    pub const fn is_lower(&self) -> bool {
        matches!(self, Self::Lower(_))
    }

    #[must_use]
    pub const fn is_lower_or_equal(&self) -> bool {
        matches!(self, Self::LowerOrEqual(_))
    }

    #[must_use]
    pub const fn is_exact(&self) -> bool {
        matches!(self, Self::Exact(_))
    }

    #[must_use]
    pub const fn is_higher_or_equal(&self) -> bool {
        matches!(self, Self::HigherOrEqual(_))
    }

    #[must_use]
    pub const fn is_higher(&self) -> bool {
        matches!(self, Self::Higher(_))
    }

    #[must_use]
    pub const fn version(&self) -> Option<&Version> {
        match self {
            Self::Any => None,
            Self::Lower(v)
            | Self::LowerOrEqual(v)
            | Self::Exact(v)
            | Self::HigherOrEqual(v)
            | Self::Higher(v) => Some(v),
        }
    }
}

impl From<Version> for DependencyVersion {
    fn from(version: Version) -> Self {
        Self::Exact(version)
    }
}

impl From<&Version> for DependencyVersion {
    fn from(version: &Version) -> Self {
        Self::Exact(*version)
    }
}

impl fmt::Display for DependencyVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Any => String::new(),
            Self::Lower(version) => format!(" < {version}"),
            Self::LowerOrEqual(version) => format!(" <= {version}"),
            Self::Exact(version) => format!(" = {version}"),
            Self::HigherOrEqual(version) => format!(" >= {version}"),
            Self::Higher(version) => format!(" > {version}"),
        };

        write!(f, "{s}")
    }
}

impl Serialize for DependencyVersion {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("{self}"))
    }
}

impl<'de> Deserialize<'de> for DependencyVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DependencyVersionVisitor)
    }
}

struct DependencyVersionVisitor;

impl<'de> Visitor<'de> for DependencyVersionVisitor {
    type Value = DependencyVersion;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a dependency version: <, <=, =, >=, > + version or nothing")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let v = v.trim();
        if v.is_empty() {
            return Ok(DependencyVersion::Any);
        }

        let mut parts = v.split(' ');

        let comparator = parts
            .next()
            .ok_or_else(|| Error::custom("Invalid dependency version: failed to get comparator"))?;
        let version = parts
            .next()
            .ok_or_else(|| Error::custom("Invalid dependency version: failed to get version"))?;

        let version = VersionVisitor::visit_str::<E>(VersionVisitor, version)?;

        match comparator {
            "<" => Ok(DependencyVersion::Lower(version)),
            "<=" => Ok(DependencyVersion::LowerOrEqual(version)),
            "=" => Ok(DependencyVersion::Exact(version)),
            ">=" => Ok(DependencyVersion::HigherOrEqual(version)),
            ">" => Ok(DependencyVersion::Higher(version)),
            _ => Err(Error::custom("Invalid dependency version")),
        }
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_str(&v)
    }
}

#[derive(Debug, Clone)]
pub struct Dependency {
    kind: DependencyType,
    name: String,
    version: DependencyVersion,
}

impl Dependency {
    #[must_use]
    pub const fn is_required(&self) -> bool {
        self.kind.is_required()
    }

    #[must_use]
    pub const fn is_optional(&self) -> bool {
        self.kind.is_optional()
    }

    #[must_use]
    pub const fn is_incompatible(&self) -> bool {
        self.kind.is_incompatible()
    }

    #[must_use]
    pub const fn name(&self) -> &String {
        &self.name
    }

    #[must_use]
    pub const fn version(&self) -> &DependencyVersion {
        &self.version
    }
}

pub trait DependencyUtil {
    fn allows(&self, name: &str, version: Version) -> bool;
    fn conflicts(&self, name: &str, version: Version) -> bool;
}

impl DependencyUtil for Dependency {
    fn allows(&self, name: &str, version: Version) -> bool {
        (&self).allows(name, version)
    }

    fn conflicts(&self, name: &str, version: Version) -> bool {
        (&self).conflicts(name, version)
    }
}

impl DependencyUtil for &Dependency {
    fn allows(&self, name: &str, version: Version) -> bool {
        self.name == name && !self.kind.is_incompatible() && self.version.allows(version)
    }

    fn conflicts(&self, name: &str, version: Version) -> bool {
        self.name == name && (!self.version.allows(version) || self.kind.is_incompatible())
    }
}

impl fmt::Display for Dependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = &self.kind;
        let name = &self.name;
        let version = &self.version;

        write!(f, "{prefix} {name}{version}")
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
        deserializer.deserialize_str(DependencyVisitor)
    }
}

struct DependencyVisitor;

impl<'de> Visitor<'de> for DependencyVisitor {
    type Value = Dependency;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a dependency: !, ?, (?), ~ or nothing followed by a mod name and optionally a version specifier")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        #[allow(clippy::unwrap_used)] // The regex is hardcoded and will always compile
        let extractor =
            regex::Regex::new(r"^\s*(?:(!|\?|\(\?\)|~)\s*)?([a-zA-Z\d_\-\s]{3,50})(?:\s*(?:(>=?|<=?|=)\s*(\d+(?:\.\d+)?(?:\.\d+)?)))?\s*$")
                .unwrap();

        let Some(captures) = extractor.captures(v) else {
            return Err(Error::custom(format!(
                "Invalid dependency: does not match expected format: {v}"
            )));
        };

        let kind = captures.get(1).map_or("", |k| k.as_str());
        let kind = DependencyTypeVisitor::visit_str::<E>(DependencyTypeVisitor, kind)?;

        let name = captures
            .get(2)
            .ok_or_else(|| Error::custom("Invalid dependency: failed to get name"))?
            .as_str()
            .trim()
            .to_owned();

        let version_comparator = captures.get(3).map_or("", |v| v.as_str());
        let version = captures.get(4).map_or("", |v| v.as_str());
        let version = DependencyVersionVisitor::visit_str::<E>(
            DependencyVersionVisitor,
            format!("{version_comparator} {version}").as_str(),
        )?;

        Ok(Self::Value {
            kind,
            name,
            version,
        })
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_str(&v)
    }
}

pub trait DependencyExt: Iterator + Sized {
    fn any_allows(&mut self, name: &str, version: Version) -> bool
    where
        Self::Item: DependencyUtil,
    {
        self.any(|dep| dep.allows(name, version))
    }

    fn any_conflicts(&mut self, name: &str, version: Version) -> bool
    where
        Self::Item: DependencyUtil,
    {
        self.any(|dep| dep.conflicts(name, version))
    }

    fn collect_conflicts<B>(&mut self, name: &str, version: Version) -> B
    where
        Self::Item: DependencyUtil,
        B: FromIterator<Self::Item>,
    {
        self.filter(|dep| dep.conflicts(name, version)).collect()
    }
}

impl<I: Iterator> DependencyExt for I {}
