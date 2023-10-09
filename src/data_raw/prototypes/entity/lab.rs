use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{helper, EntityWithOwnerPrototype};
use crate::data_raw::types::*;

/// [`Prototypes/LabPrototype`](https://lua-api.factorio.com/latest/prototypes/LabPrototype.html)
pub type LabPrototype = EntityWithOwnerPrototype<LabData>;

/// [`Prototypes/LabPrototype`](https://lua-api.factorio.com/latest/prototypes/LabPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct LabData {
    pub energy_usage: Energy,
    pub energy_source: AnyEnergySource,
    pub on_animation: Animation,
    pub off_animation: Animation,
    pub inputs: Vec<ItemID>,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub researching_speed: f64,

    pub allowed_effects: Option<EffectTypeLimitation>,
    pub light: Option<LightDefinition>,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub base_productivity: f64,

    pub entity_info_icon_shift: Option<Vector>,
    pub module_specification: Option<ModuleSpecification>,
}
