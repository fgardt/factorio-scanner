#![forbid(unsafe_code)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Common {
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
    pub info: Common,

    pub default_value: bool,

    pub forced_value: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct IntSetting {
    #[serde(flatten)]
    pub info: Common,

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
    pub info: Common,

    pub default_value: f64,

    pub minimum_value: Option<f64>,
    pub maximum_value: Option<f64>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_values: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringSetting {
    #[serde(flatten)]
    pub info: Common,

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
    pub info: Common,

    pub default_value: Color,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
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
