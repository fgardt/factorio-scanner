use core::fmt;
use std::{num::ParseIntError, str::FromStr};

use serde::{
    de::{Error as DeError, Visitor},
    Deserialize, Deserializer, Serialize,
};
use serde_helper as helper;
use serde_with::{skip_serializing_none, with_suffix};
use thiserror::Error;

use crate::UsedMods;

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

    #[serde(default = "default_dep", skip_serializing_if = "is_default_dep")]
    pub dependencies: Vec<Dependency>,

    #[serde(flatten, with = "suffix_required")]
    pub flags: FeatureFlags,
}

with_suffix!(suffix_required "_required");

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct FeatureFlags {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub quality: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub rail_bridges: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub space_travel: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub spoiling: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub freezing: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub segmented_units: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub expansion_shaders: bool,
}

impl std::ops::BitOr for FeatureFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self {
            quality: self.quality || rhs.quality,
            rail_bridges: self.rail_bridges || rhs.rail_bridges,
            space_travel: self.space_travel || rhs.space_travel,
            spoiling: self.spoiling || rhs.spoiling,
            freezing: self.freezing || rhs.freezing,
            segmented_units: self.segmented_units || rhs.segmented_units,
            expansion_shaders: self.expansion_shaders || rhs.expansion_shaders,
        }
    }
}

impl std::ops::BitOrAssign for FeatureFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.quality |= rhs.quality;
        self.rail_bridges |= rhs.rail_bridges;
        self.space_travel |= rhs.space_travel;
        self.spoiling |= rhs.spoiling;
        self.freezing |= rhs.freezing;
        self.segmented_units |= rhs.segmented_units;
        self.expansion_shaders |= rhs.expansion_shaders;
    }
}

impl std::ops::BitAnd for FeatureFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self {
            quality: self.quality && rhs.quality,
            rail_bridges: self.rail_bridges && rhs.rail_bridges,
            space_travel: self.space_travel && rhs.space_travel,
            spoiling: self.spoiling && rhs.spoiling,
            freezing: self.freezing && rhs.freezing,
            segmented_units: self.segmented_units && rhs.segmented_units,
            expansion_shaders: self.expansion_shaders && rhs.expansion_shaders,
        }
    }
}

impl std::ops::BitAndAssign for FeatureFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.quality &= rhs.quality;
        self.rail_bridges &= rhs.rail_bridges;
        self.space_travel &= rhs.space_travel;
        self.spoiling &= rhs.spoiling;
        self.freezing &= rhs.freezing;
        self.segmented_units &= rhs.segmented_units;
        self.expansion_shaders &= rhs.expansion_shaders;
    }
}

impl std::ops::BitXor for FeatureFlags {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        Self {
            quality: self.quality ^ rhs.quality,
            rail_bridges: self.rail_bridges ^ rhs.rail_bridges,
            space_travel: self.space_travel ^ rhs.space_travel,
            spoiling: self.spoiling ^ rhs.spoiling,
            freezing: self.freezing ^ rhs.freezing,
            segmented_units: self.segmented_units ^ rhs.segmented_units,
            expansion_shaders: self.expansion_shaders ^ rhs.expansion_shaders,
        }
    }
}

impl std::ops::BitXorAssign for FeatureFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.quality ^= rhs.quality;
        self.rail_bridges ^= rhs.rail_bridges;
        self.space_travel ^= rhs.space_travel;
        self.spoiling ^= rhs.spoiling;
        self.freezing ^= rhs.freezing;
        self.segmented_units ^= rhs.segmented_units;
        self.expansion_shaders ^= rhs.expansion_shaders;
    }
}

fn default_dep() -> Vec<Dependency> {
    vec![Dependency {
        kind: DependencyType::Required,
        name: "base".to_owned(),
        version: DependencyVersion::Any,
    }]
}

fn is_default_dep(input: &[Dependency]) -> bool {
    input.len() == 1
        && input[0].name == "base"
        && input[0].version.is_any()
        && input[0].kind.is_required()
}

impl ModInfo {
    #[must_use]
    pub fn dependency_chain_length(&self, used: &UsedMods) -> usize {
        // core is always first
        if self.name == "core" {
            return 0;
        }

        let mut max = 0;
        for dep in &self.dependencies {
            if !dep.affects_load_order() {
                continue;
            }

            let Some(dep_mod) = used.get(dep.name()) else {
                continue;
            };

            // this can go into a loop if there are circular dependencies
            // but the dependency resolver should prevent that
            let len = dep_mod.info.dependency_chain_length(used);
            if len > max {
                max = len;
            }
        }

        max + 1
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Version {
    major: u16,
    minor: u16,
    patch: u16,

    leading_major: u8,
    leading_minor: u8,
    leading_patch: u8,
}

impl Version {
    #[must_use]
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
            leading_major: 0,
            leading_minor: 0,
            leading_patch: 0,
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

        let p1 = usize::from(self.leading_major) + major.checked_ilog10().unwrap_or(0) as usize + 1;
        let p2 = usize::from(self.leading_minor) + minor.checked_ilog10().unwrap_or(0) as usize + 1;
        let p3 = usize::from(self.leading_patch) + patch.checked_ilog10().unwrap_or(0) as usize + 1;

        write!(f, "{major:0>p1$}.{minor:0>p2$}.{patch:0>p3$}")
    }
}

#[derive(Debug, Error)]
pub enum VersionParseError {
    #[error("invalid version format: expected \"major.minor.patch\"")]
    InvalidFormat,

    #[error("invalid version number: {0}")]
    InvalidNumber(String),
}

impl FromStr for Version {
    type Err = VersionParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::try_from(value)
    }
}

fn parse_version_part(part: &str) -> Result<(u16, u8), VersionParseError> {
    let value = part
        .parse()
        .map_err(|err: ParseIntError| VersionParseError::InvalidNumber(err.to_string()))?;

    let mut leading = part.chars().take_while(|&c| c == '0').count() as u8;

    if value == 0 {
        leading -= 1;
    }

    Ok((value, leading))
}

impl TryFrom<&str> for Version {
    type Error = VersionParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split('.');

        let major_raw = parts.next().ok_or(VersionParseError::InvalidFormat)?;
        let minor_raw = parts.next().ok_or(VersionParseError::InvalidFormat)?;
        let patch_raw = parts.next().ok_or(VersionParseError::InvalidFormat)?;

        let (major, leading_major) = parse_version_part(major_raw)?;
        let (minor, leading_minor) = parse_version_part(minor_raw)?;
        let (patch, leading_patch) = parse_version_part(patch_raw)?;

        Ok(Self {
            major,
            minor,
            patch,
            leading_major,
            leading_minor,
            leading_patch,
        })
    }
}

impl TryFrom<String> for Version {
    type Error = VersionParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&String> for Version {
    type Error = VersionParseError;

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

impl Visitor<'_> for VersionVisitor {
    type Value = Version;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(
            "a version in the format \"major.minor.patch\" where each part is a u16 or a single packed u64",
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        let mut parts = v.split('.');

        // parts will default to 0 if missing
        let mut major_raw = parts.next().unwrap_or("0");
        let mut minor_raw = parts.next().unwrap_or("0");
        let mut patch_raw = parts.next().unwrap_or("0");

        if major_raw.is_empty() {
            major_raw = "0";
        }

        if minor_raw.is_empty() {
            minor_raw = "0";
        }

        if patch_raw.is_empty() {
            patch_raw = "0";
        }

        let (major, leading_major) = parse_version_part(major_raw).map_err(DeError::custom)?;
        let (minor, leading_minor) = parse_version_part(minor_raw).map_err(DeError::custom)?;
        let (patch, leading_patch) = parse_version_part(patch_raw).map_err(DeError::custom)?;

        Ok(Self::Value {
            major,
            minor,
            patch,
            leading_major,
            leading_minor,
            leading_patch,
        })
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: DeError,
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

    pub const fn affects_load_order(&self) -> bool {
        matches!(self, Self::Required | Self::Optional | Self::HiddenOptional)
    }
}

impl fmt::Display for DependencyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Incompatible => write!(f, "! "),
            Self::Optional => write!(f, "? "),
            Self::HiddenOptional => write!(f, "(?) "),
            Self::RequiredLazy => write!(f, "~ "),
            Self::Required => Ok(()),
        }
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

    #[must_use]
    pub fn get_allowed_version<'a>(&self, versions: &'a [Version]) -> Option<&'a Version> {
        match self {
            Self::Any => versions.iter().max(),
            Self::Lower(v) => versions.iter().filter(|&x| x < v).max(),
            Self::LowerOrEqual(v) => versions.iter().filter(|&x| x <= v).max(),
            Self::Exact(v) => versions.iter().find(|&x| x == v),
            Self::HigherOrEqual(v) => versions.iter().filter(|&x| x >= v).max(),
            Self::Higher(v) => versions.iter().filter(|&x| x > v).max(),
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
        match self {
            Self::Any => Ok(()),
            Self::Lower(version) => write!(f, " < {version}"),
            Self::LowerOrEqual(version) => write!(f, " <= {version}"),
            Self::Exact(version) => write!(f, " = {version}"),
            Self::HigherOrEqual(version) => write!(f, " >= {version}"),
            Self::Higher(version) => write!(f, " > {version}"),
        }
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
    pub const fn affects_load_order(&self) -> bool {
        self.kind.affects_load_order()
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

impl std::str::FromStr for Dependency {
    type Err = serde::de::value::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DependencyVisitor::visit_str(DependencyVisitor, s)
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

        write!(f, "{prefix}{name}{version}")
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

impl Visitor<'_> for DependencyVisitor {
    type Value = Dependency;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a dependency: !, ?, (?), ~ or nothing followed by a mod name and optionally a version specifier")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        let mut v = v.trim();

        if v.is_empty() {
            return Err(DeError::custom("Invalid dependency: empty string"));
        }

        let kind = if v.starts_with('!') {
            v = &v[1..];
            DependencyType::Incompatible
        } else if v.starts_with('?') {
            v = &v[1..];
            DependencyType::Optional
        } else if v.starts_with("(?)") {
            v = &v[3..];
            DependencyType::HiddenOptional
        } else if v.starts_with('~') {
            v = &v[1..];
            DependencyType::RequiredLazy
        } else {
            DependencyType::Required
        };

        v = v.trim_start();

        let mut version = DependencyVersion::Any;
        let name = if let Some(comp_start) = v.find(['<', '>', '=']) {
            let (name, ver) = v.split_at(comp_start);

            if ver.len() >= 2 {
                let mut ver_c = ver.chars();
                let Some(first) = ver_c.next() else {
                    return Err(DeError::custom("Invalid dependency version comparator"));
                };
                let Some(second) = ver_c.next() else {
                    return Err(DeError::custom("Invalid dependency version comparator"));
                };

                version = match (first, second) {
                    ('>', '=') | ('=', '>') => DependencyVersion::HigherOrEqual(
                        VersionVisitor::visit_str(VersionVisitor, ver[2..].trim_start())?,
                    ),
                    ('<', '=') | ('=', '<') => DependencyVersion::LowerOrEqual(
                        VersionVisitor::visit_str(VersionVisitor, ver[2..].trim_start())?,
                    ),
                    ('=', '=') => DependencyVersion::Exact(VersionVisitor::visit_str(
                        VersionVisitor,
                        ver[2..].trim_start(),
                    )?),
                    ('=', _) => DependencyVersion::Exact(VersionVisitor::visit_str(
                        VersionVisitor,
                        ver[1..].trim_start(),
                    )?),
                    ('>', _) => DependencyVersion::Higher(VersionVisitor::visit_str(
                        VersionVisitor,
                        ver[1..].trim_start(),
                    )?),
                    ('<', _) => DependencyVersion::Lower(VersionVisitor::visit_str(
                        VersionVisitor,
                        ver[1..].trim_start(),
                    )?),
                    _ => return Err(DeError::custom("Invalid dependency version comparator")),
                };
            }

            name.trim_end()
        } else {
            v
        };

        Ok(Self::Value {
            kind,
            name: name.to_string(),
            version,
        })
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: DeError,
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

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn version_print() {
        let version = Version {
            major: 1,
            minor: 2,
            patch: 3,
            leading_major: 1,
            leading_minor: 2,
            leading_patch: 3,
        };

        assert_eq!(format!("{version}"), "01.002.0003");
    }

    #[test]
    fn version_parse() {
        let input = "01.002.0003";
        let version = Version::try_from(input).unwrap();
        assert_eq!(format!("{version}"), input);
    }

    #[test]
    fn version_zero() {
        let input = "0.0.0";
        let version = Version::try_from(input).unwrap();
        assert_eq!(format!("{version}"), input);
    }

    #[allow(clippy::needless_pass_by_value)]
    fn dep_test(input: &str, kind: DependencyType, name: &str, version: DependencyVersion) {
        let dep = DependencyVisitor::visit_str::<serde::de::value::Error>(DependencyVisitor, input)
            .unwrap();

        assert_eq!(dep.kind, kind);
        assert_eq!(dep.name, name);
        assert_eq!(dep.version, version);
    }

    #[test]
    fn dep_simple() {
        dep_test(
            "mod-name",
            DependencyType::Required,
            "mod-name",
            DependencyVersion::Any,
        );
    }

    #[test]
    fn dep_with_whitespace() {
        dep_test(
            "legacy mod name",
            DependencyType::Required,
            "legacy mod name",
            DependencyVersion::Any,
        );
    }

    #[test]
    fn dep_version() {
        dep_test(
            "mod-name = 1.0.0",
            DependencyType::Required,
            "mod-name",
            DependencyVersion::Exact(Version::new(1, 0, 0)),
        );
    }

    #[test]
    fn dep_partial_version() {
        dep_test(
            "mod-name >= 1",
            DependencyType::Required,
            "mod-name",
            DependencyVersion::HigherOrEqual(Version::new(1, 0, 0)),
        );
    }

    #[test]
    fn dep_optional() {
        dep_test(
            "? mod-name",
            DependencyType::Optional,
            "mod-name",
            DependencyVersion::Any,
        );
    }

    #[test]
    fn dep_lazy() {
        dep_test(
            "~ mod-name",
            DependencyType::RequiredLazy,
            "mod-name",
            DependencyVersion::Any,
        );
    }

    #[test]
    fn dep_incompatible() {
        dep_test(
            "! mod-name",
            DependencyType::Incompatible,
            "mod-name",
            DependencyVersion::Any,
        );
    }

    #[test]
    fn dep_hidden_optional_version() {
        dep_test(
            "(?) mod-name <= 0.4.5",
            DependencyType::HiddenOptional,
            "mod-name",
            DependencyVersion::LowerOrEqual(Version::new(0, 4, 5)),
        );
    }

    #[test]
    fn dep_compact_lazy_partial_version() {
        dep_test(
            "~helloworld>2",
            DependencyType::RequiredLazy,
            "helloworld",
            DependencyVersion::Higher(Version::new(2, 0, 0)),
        );
    }

    #[test]
    fn dep_compact_with_whitespace() {
        dep_test(
            "(?)hello world>3.4.5",
            DependencyType::HiddenOptional,
            "hello world",
            DependencyVersion::Higher(Version::new(3, 4, 5)),
        );
    }

    #[test]
    fn dep_leading_zero() {
        dep_test(
            "!mod_name=0035.042.001337",
            DependencyType::Incompatible,
            "mod_name",
            DependencyVersion::Exact(Version {
                major: 35,
                minor: 42,
                patch: 1337,
                leading_major: 2,
                leading_minor: 1,
                leading_patch: 2,
            }),
        );
    }

    #[test]
    fn dep_dot() {
        dep_test(
            "name=.",
            DependencyType::Required,
            "name",
            DependencyVersion::Exact(Version::new(0, 0, 0)),
        );
    }

    #[test]
    fn dep_double_equal() {
        dep_test(
            "name==0.0.0",
            DependencyType::Required,
            "name",
            DependencyVersion::Exact(Version::new(0, 0, 0)),
        );
    }

    #[test]
    fn dep_higher() {
        dep_test(
            "name>0.0.0",
            DependencyType::Required,
            "name",
            DependencyVersion::Higher(Version::new(0, 0, 0)),
        );
    }

    #[test]
    fn dep_higher_equal() {
        dep_test(
            "name>=0.0.0",
            DependencyType::Required,
            "name",
            DependencyVersion::HigherOrEqual(Version::new(0, 0, 0)),
        );
    }

    #[test]
    fn dep_higher_equal_flipped() {
        dep_test(
            "name=>0.0.0",
            DependencyType::Required,
            "name",
            DependencyVersion::HigherOrEqual(Version::new(0, 0, 0)),
        );
    }

    #[test]
    fn dep_lower() {
        dep_test(
            "name<0.0.0",
            DependencyType::Required,
            "name",
            DependencyVersion::Lower(Version::new(0, 0, 0)),
        );
    }

    #[test]
    fn dep_lower_equal() {
        dep_test(
            "name<=0.0.0",
            DependencyType::Required,
            "name",
            DependencyVersion::LowerOrEqual(Version::new(0, 0, 0)),
        );
    }

    #[test]
    fn dep_lower_equal_flipped() {
        dep_test(
            "name=<0.0.0",
            DependencyType::Required,
            "name",
            DependencyVersion::LowerOrEqual(Version::new(0, 0, 0)),
        );
    }
}
