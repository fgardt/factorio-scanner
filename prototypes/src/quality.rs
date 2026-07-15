use serde::{Deserialize, Serialize};

use serde_helper as helper;
use serde_with::{skip_serializing_none, with_suffix};
use types::{Color, Icon, ItemStackIndex, QualityID};

use crate::helper_macro::namespace_struct;

/// [`Prototypes/QualityPrototype`](https://lua-api.factorio.com/latest/prototypes/QualityPrototype.html)
pub type QualityPrototype = crate::BasePrototype<QualityPrototypeData>;

/// [`Prototypes/QualityPrototype`](https://lua-api.factorio.com/latest/prototypes/QualityPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct QualityPrototypeData {
    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_sprite_by_default: bool,

    pub color: Color,
    pub level: u32,
    pub next: Option<QualityID>,

    #[serde(flatten, with = "suffix_probability")]
    pub probabilities: Probabilities,

    #[serde(flatten)]
    pub icon: Icon,

    #[serde(flatten, with = "suffix_multiplier")]
    pub multipliers: Box<Multipliers>,

    #[serde(flatten, with = "suffix_bonus")]
    pub bonus: Box<Bonus>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct Probabilities {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub next: f64,
    pub chain: Option<f64>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub previous: f64,
    pub previous_chain: Option<f64>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct Multipliers {
    #[serde(default = "helper::f32_1", skip_serializing_if = "helper::is_1_f32")]
    pub beacon_power_usage: f32,

    #[serde(default = "helper::f32_1", skip_serializing_if = "helper::is_1_f32")]
    pub mining_drill_resource_drain: f32,

    #[serde(default = "helper::f32_1", skip_serializing_if = "helper::is_1_f32")]
    pub science_pack_drain: f32,

    pub default: Option<f64>,
    pub inserter_speed: Option<f64>,
    pub fluid_wagon_capacity: Option<f64>,
    pub inventory_size: Option<f64>,

    pub cargo_wagon_inventory_size: Option<f64>,
    pub locomotive_power: Option<f64>,
    pub rolling_stock_max_speed: Option<f64>,

    pub lab_research_speed: Option<f64>,
    pub crafting_machine_speed: Option<f64>,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub crafting_machine_energy_usage: f64,

    pub logistic_cell_charging_energy: Option<f64>,
    pub tool_durability: Option<f64>,
    pub accumulator_capacity: Option<f64>,
    pub flying_robot_max_energy: Option<f64>,
    pub range: Option<f64>,

    pub module_consumption: Option<f32>,
    pub module_speed: Option<f32>,
    pub module_productivity: Option<f32>,
    pub module_pollution: Option<f32>,
    pub module_quality: Option<f32>,

    pub spoil_ticks: Option<f32>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct Bonus {
    pub asteroid_collector_collection_radius: Option<f64>,
    pub equipment_grid_width: Option<i16>,
    pub equipment_grid_height: Option<i16>,
    pub electric_pole_wire_reach: Option<f32>,
    pub electric_pole_supply_area_distance: Option<f32>,
    pub beacon_supply_area_distance: Option<f32>,
    pub mining_drill_mining_radius: Option<f32>,
    pub logistic_cell_charging_station_count: Option<u32>,
    pub beacon_module_slots: Option<ItemStackIndex>,
    pub crafting_machine_module_slots: Option<ItemStackIndex>,
    pub mining_drill_module_slots: Option<ItemStackIndex>,
    pub lab_module_slots: Option<ItemStackIndex>,
}

with_suffix!(suffix_probability "_probability");
with_suffix!(suffix_multiplier "_multiplier");
with_suffix!(suffix_bonus "_bonus");

namespace_struct! {
    AllTypes,
    QualityID,
    "quality"
}

impl AllTypes {
    #[cfg(feature = "graphics")]
    pub fn get_icon(
        &self,
        name: &str,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut types::ImageCache,
    ) -> Option<types::GraphicsOutput> {
        use types::RenderableGraphics;

        self.quality
            .get(&QualityID::new(name))
            .and_then(|proto| proto.icon.render(scale, used_mods, image_cache, &()))
    }
}
