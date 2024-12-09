use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use crate::FactorioArray;

/// [`Types/EffectValue`](https://lua-api.factorio.com/latest/types/EffectValue.html)
pub type EffectValue = f64;

/// [`Types/Effect`](https://lua-api.factorio.com/latest/types/Effect.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Effect {
    pub consumption: Option<EffectValue>,
    pub speed: Option<EffectValue>,
    pub productivity: Option<EffectValue>,
    pub pollution: Option<EffectValue>,
    pub quality: Option<EffectValue>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EffectType {
    Speed,
    Productivity,
    Consumption,
    Pollution,
    Quality,
}

/// [`Types/EffectTypeLimitation`](https://lua-api.factorio.com/latest/types/EffectTypeLimitation.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EffectTypeLimitation {
    Single(EffectType),
    Multiple(FactorioArray<EffectType>),
}

/// [`Types/EffectReceiver`](https://lua-api.factorio.com/latest/types/EffectReceiver.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct EffectReceiver {
    pub base_effect: Option<Effect>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub uses_module_effects: bool,
    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub uses_beacon_effects: bool,
    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub uses_surface_effects: bool,
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
