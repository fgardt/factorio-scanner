use std::{
    collections::HashMap,
    fs,
    io::{Cursor, Seek, SeekFrom},
    path::Path,
};

use anyhow::{anyhow, Result};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

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
pub struct SettingsDat<'a> {
    path: &'a Path,

    pub version: u64, // https://wiki.factorio.com/Version_string_format

    pub startup: HashMap<String, PropertyTree>,
    pub runtime_global: HashMap<String, PropertyTree>,
    pub runtime_per_user: HashMap<String, PropertyTree>,
}

impl<'a> SettingsDat<'a> {
    pub fn load(path: &'a Path) -> Result<Self> {
        let mut cursor = Cursor::new(fs::read(path)?);
        let version = cursor.read_u64::<LittleEndian>()?;
        cursor.seek(SeekFrom::Current(1))?; // skip false bool

        let data = PropertyTree::load(&mut cursor)?;

        let PropertyTree::Dictionary(data) = data else {
            return Err(anyhow!("Invalid settings.dat: not a dictionary"));
        };

        let Some(PropertyTree::Dictionary(startup)) = data.get("startup") else {
            return Err(anyhow!("Invalid settings.dat: no startup tree"));
        };

        let Some(PropertyTree::Dictionary(rt_g)) = data.get("runtime-global") else {
            return Err(anyhow!("Invalid settings.dat: no runtime-global tree"));
        };

        let Some(PropertyTree::Dictionary(rt_p_u)) = data.get("runtime-per-user") else {
            return Err(anyhow!("Invalid settings.dat: no runtime-per-user tree"));
        };

        Ok(Self {
            path,
            version,
            startup: startup.clone(),
            runtime_global: rt_g.clone(),
            runtime_per_user: rt_p_u.clone(),
        })
    }

    pub fn write(&self, path: &Path) -> Result<()> {
        let mut buf = Vec::new();

        buf.write_u64::<LittleEndian>(self.version)?;
        buf.write_u8(0)?; // false bool

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

        data.write(&mut buf)?;
        fs::write(path, buf)?;

        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        self.write(self.path)
    }

    #[cfg(feature = "bp_meta_info")]
    pub fn load_bp_settings(
        settings: &crate::TagTable,
        version: u64,
        path: &'a Path,
    ) -> Result<Self> {
        let mut startup = HashMap::new();

        for (k, v) in settings {
            let pt = settings_property_tree(v)?;
            startup.insert(k.clone(), pt);
        }

        Ok(Self {
            path,
            version,
            startup,
            runtime_global: HashMap::new(),
            runtime_per_user: HashMap::new(),
        })
    }
}

#[cfg(feature = "bp_meta_info")]
fn settings_property_tree(value: &crate::AnyBasic) -> Result<PropertyTree> {
    use crate::AnyBasic;

    let pt_val = match value {
        AnyBasic::Bool(val) => PropertyTree::Bool(*val),
        AnyBasic::Number(val) => PropertyTree::Number(*val),
        AnyBasic::String(val) => PropertyTree::String(val.clone()),
        AnyBasic::Table(val) => {
            let mut map = HashMap::new();

            for (k, v) in val {
                let AnyBasic::Number(num) = v else {
                    return Err(anyhow!(
                        "Invalid settings in blueprint: expected number in color setting"
                    ));
                };

                map.insert(k.clone(), PropertyTree::Number(*num));
            }

            PropertyTree::Dictionary(map)
        }
        AnyBasic::Array(_) => {
            return Err(anyhow!("Invalid settings in blueprint: unexpected array"))
        }
    };

    Ok(PropertyTree::Dictionary(
        vec![("value".to_owned(), pt_val)].into_iter().collect(),
    ))
}
