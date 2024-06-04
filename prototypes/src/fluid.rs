use serde::{Deserialize, Serialize};

use serde_helper as helper;
use types::{Color, Energy, FluidID, Icon, ItemSubGroupID, RenderableGraphics};

use crate::helper_macro::namespace_struct;

/// [`Prototypes/FluidPrototype`](https://lua-api.factorio.com/latest/prototypes/FluidPrototype.html)
pub type FluidPrototype = crate::BasePrototype<FluidPrototypeData>;

/// [`Prototypes/FluidPrototype`](https://lua-api.factorio.com/latest/prototypes/FluidPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct FluidPrototypeData {
    #[serde(flatten)]
    pub icon: Icon,

    pub default_temperature: f64,

    #[serde(
        default,
        deserialize_with = "helper::inf_float_opt_deserializer",
        serialize_with = "helper::inf_float_opt_serializer",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_temperature: Option<f64>,

    #[serde(
        default = "helper::f64_max",
        deserialize_with = "helper::inf_float_deserializer",
        serialize_with = "helper::inf_float_serializer",
        skip_serializing_if = "helper::is_max_f64"
    )]
    pub gas_temperature: f64,

    pub base_color: Color,
    pub flow_color: Color,

    #[serde(
        default = "default_capacity",
        skip_serializing_if = "is_default_capacity"
    )]
    pub heat_capacity: Energy,

    #[serde(
        default = "default_fuel_value",
        skip_serializing_if = "is_default_fuel_value"
    )]
    pub fuel_value: Energy,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub emissions_multiplier: f64,

    #[serde(
        default = "default_subgroup",
        skip_serializing_if = "is_default_subgroup"
    )]
    pub subgroup: ItemSubGroupID,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub hidden: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub auto_barrel: bool,
}

impl FluidPrototypeData {
    pub fn get_icon(
        &self,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut types::ImageCache,
    ) -> Option<types::GraphicsOutput> {
        self.icon.render(scale, used_mods, image_cache, &())
    }
}

fn default_capacity() -> Energy {
    "1KJ".to_owned()
}

fn default_fuel_value() -> Energy {
    "0J".to_owned()
}

fn default_subgroup() -> ItemSubGroupID {
    ItemSubGroupID::new("fluid")
}

fn is_default_capacity(capacity: &Energy) -> bool {
    *capacity == default_capacity()
}

fn is_default_fuel_value(fuel_value: &Energy) -> bool {
    *fuel_value == default_fuel_value()
}

fn is_default_subgroup(subgroup: &ItemSubGroupID) -> bool {
    *subgroup == default_subgroup()
}

namespace_struct! {
    AllTypes,
    FluidID,
    "fluid"
}

impl AllTypes {
    pub fn get_icon(
        &self,
        name: &str,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut types::ImageCache,
    ) -> Option<types::GraphicsOutput> {
        self.fluid
            .get(&FluidID::new(name))
            .and_then(|proto| proto.get_icon(scale, used_mods, image_cache))
    }
}
