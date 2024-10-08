use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use crate::{FactorioArray, ItemStackIndex, Vector};

/// [`Types/ModuleSpecification`](https://lua-api.factorio.com/latest/types/ModuleSpecification.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleSpecification {
    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub module_slots: ItemStackIndex,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "helper::truncating_opt_deserializer"
    )]
    pub module_info_max_icons_per_row: Option<u8>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "helper::truncating_opt_deserializer"
    )]
    pub module_info_max_icon_rows: Option<u8>,
    pub module_info_icon_shift: Option<Vector>,
    pub module_info_separation_multiplier: Option<f32>,
    pub module_info_multi_row_initial_height_modifier: Option<f32>,
}

/// [`Types/EffectValue`](https://lua-api.factorio.com/latest/types/EffectValue.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct EffectValue {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub bonus: f64,
}

/// [`Types/Effect`](https://lua-api.factorio.com/latest/types/Effect.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct Effect {
    pub consumption: Option<EffectValue>,
    pub speed: Option<EffectValue>,
    pub productivity: Option<EffectValue>,
    pub pollution: Option<EffectValue>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EffectType {
    Speed,
    Productivity,
    Consumption,
    Pollution,
}

/// [`Types/EffectTypeLimitation`](https://lua-api.factorio.com/latest/types/EffectTypeLimitation.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EffectTypeLimitation {
    Single(EffectType),
    Multiple(FactorioArray<EffectType>),
}

/// [`Types/ModuleTint`](https://lua-api.factorio.com/latest/types/ModuleTint.html)
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ModuleTint {
    #[default]
    None,
    Primary,
    Secondary,
    Tertiary,
    Quaternary,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ModuleTintMode {
    #[default]
    SingleModule,
    Mix,
}
