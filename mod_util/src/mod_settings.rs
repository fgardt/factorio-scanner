use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use crate::property_tree::PropertyTree;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct CommonSettingsData {
    pub name: String,
    pub setting_type: String, // todo: enum

    pub localised_name: Option<String>,
    pub localised_description: Option<String>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub order: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct BoolSetting {
    #[serde(flatten)]
    pub info: CommonSettingsData,

    pub default_value: bool,

    pub forced_value: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct IntSetting {
    #[serde(flatten)]
    pub info: CommonSettingsData,

    pub default_value: i64,

    pub minimum_value: Option<i64>,
    pub maximum_value: Option<i64>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_values: Vec<i64>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct DoubleSetting {
    #[serde(flatten)]
    pub info: CommonSettingsData,

    pub default_value: f64,

    pub minimum_value: Option<f64>,
    pub maximum_value: Option<f64>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_values: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringSetting {
    #[serde(flatten)]
    pub info: CommonSettingsData,

    pub default_value: String,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub allow_blank: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub auto_trim: bool,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_values: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Color {
    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub r: f64,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub g: f64,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub b: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub a: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColorSetting {
    #[serde(flatten)]
    pub info: CommonSettingsData,

    pub default_value: Color,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModSettings {
    #[serde(
        rename = "bool-setting",
        default,
        skip_serializing_if = "HashMap::is_empty"
    )]
    pub bool_settings: HashMap<String, BoolSetting>,

    #[serde(
        rename = "int-setting",
        default,
        skip_serializing_if = "HashMap::is_empty"
    )]
    pub int_settings: HashMap<String, IntSetting>,

    #[serde(
        rename = "double-setting",
        default,
        skip_serializing_if = "HashMap::is_empty"
    )]
    pub double_settings: HashMap<String, DoubleSetting>,

    #[serde(
        rename = "string-setting",
        default,
        skip_serializing_if = "HashMap::is_empty"
    )]
    pub string_settings: HashMap<String, StringSetting>,

    #[serde(
        rename = "color-setting",
        default,
        skip_serializing_if = "HashMap::is_empty"
    )]
    pub color_settings: HashMap<String, ColorSetting>,
}

#[derive(Debug, Clone)]
pub struct SettingsDat {
    pub version: u64, // https://wiki.factorio.com/Version_string_format

    pub startup: HashMap<String, PropertyTree>,
    pub runtime_global: HashMap<String, PropertyTree>,
    pub runtime_per_user: HashMap<String, PropertyTree>,
}

impl SettingsDat {
    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let version_buff = [
            *bytes.first()?,
            *bytes.get(1)?,
            *bytes.get(2)?,
            *bytes.get(3)?,
            *bytes.get(4)?,
            *bytes.get(5)?,
            *bytes.get(6)?,
            *bytes.get(7)?,
        ];
        let version = u64::from_le_bytes(version_buff);
        //let false_bool: bool = *bytes.get(8)? == 0; // always false bool

        let data = PropertyTree::from_bytes(&bytes[9..])?;

        let PropertyTree::Dictionary(data) = data else {
            return None;
        };

        let PropertyTree::Dictionary(startup) = data.get("startup")?.clone() else {
            return None;
        };

        let PropertyTree::Dictionary(runtime_global) = data.get("runtime-global")?.clone() else {
            return None;
        };

        let PropertyTree::Dictionary(runtime_per_user) = data.get("runtime-per-user")?.clone()
        else {
            return None;
        };

        Some(Self {
            version,
            startup,
            runtime_global,
            runtime_per_user,
        })
    }

    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.version.to_le_bytes());
        bytes.push(0); // always false bool

        let data = PropertyTree::Dictionary(
            vec![
                (
                    "startup".to_owned(),
                    PropertyTree::Dictionary(self.startup.clone()),
                ),
                (
                    "runtime-global".to_owned(),
                    PropertyTree::Dictionary(self.runtime_global.clone()),
                ),
                (
                    "runtime-per-user".to_owned(),
                    PropertyTree::Dictionary(self.runtime_per_user.clone()),
                ),
            ]
            .iter()
            .cloned()
            .collect(),
        );

        bytes.extend(data.to_bytes());

        bytes
    }
}
