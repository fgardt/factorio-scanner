use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::{EntityWithOwnerPrototype, WireEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/RoboportPrototype`](https://lua-api.factorio.com/latest/prototypes/RoboportPrototype.html)
pub type RoboportPrototype = EntityWithOwnerPrototype<WireEntityData<RoboportData>>;

/// [`Prototypes/RoboportPrototype`](https://lua-api.factorio.com/latest/prototypes/RoboportPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RoboportData {
    pub energy_source: AnyEnergySource, // electric or void
    pub energy_usage: Energy,
    pub recharge_minimum: Energy,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub robot_slots_count: ItemStackIndex,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub material_slots_count: ItemStackIndex,

    pub base: Sprite,
    pub base_patch: Sprite,
    pub base_animation: Animation,
    pub door_animation_up: Animation,
    pub door_animation_down: Animation,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub request_to_open_door_timeout: u32,

    pub recharging_animation: Animation,
    pub spawn_and_station_height: f64, // docs specify single precision float
    pub charge_approach_distance: f64, // docs specify single precision float
    pub logistics_radius: f64,         // docs specify single precision float
    pub construction_radius: f64,      // docs specify single precision float
    pub charging_energy: Energy,

    pub default_available_logistic_output_signal: Option<SignalIDConnector>,
    pub default_total_logistic_output_signal: Option<SignalIDConnector>,
    pub default_available_construction_output_signal: Option<SignalIDConnector>,
    pub default_total_construction_output_signal: Option<SignalIDConnector>,

    // docs specify single precision float
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub spawn_and_station_shadow_height_offset: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_logistic_radius_visualization: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_construction_radius_visualization: bool,

    pub recharging_light: Option<LightDefinition>,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub charging_station_count: u32,

    // docs specify single precision float
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub charging_distance: f64,

    pub charging_station_shift: Option<Vector>,

    // docs specify single precision float, unused
    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub charging_threshold_distance: f64,

    // docs specify single precision float
    #[serde(
        default = "helper::f64_001",
        skip_serializing_if = "helper::is_001_f64"
    )]
    pub robot_vertical_acceleration: f64,

    pub stationing_offset: Option<Vector>,

    // unused
    pub robot_limit: Option<ItemCountType>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub robots_shrink_when_entering_and_exiting: bool,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub charging_offsets: FactorioArray<Vector>,

    // docs specify single precision float
    pub logistics_connection_distance: Option<f64>,
    // not implemented
    // pub open_door_trigger_effect: Option<TriggerEffect>,
    // pub close_door_trigger_effect: Option<TriggerEffect>,
}

impl super::Renderable for RoboportData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = merge_renders(
            &[
                self.base.render(
                    render_layers.scale(),
                    used_mods,
                    image_cache,
                    &options.into(),
                ),
                self.base_animation.render(
                    render_layers.scale(),
                    used_mods,
                    image_cache,
                    &options.into(),
                ),
                self.door_animation_up.render(
                    render_layers.scale(),
                    used_mods,
                    image_cache,
                    &options.into(),
                ),
                self.door_animation_down.render(
                    render_layers.scale(),
                    used_mods,
                    image_cache,
                    &options.into(),
                ),
            ],
            render_layers.scale(),
        )?;

        render_layers.add_entity(res, &options.position);

        Some(())

        // TODO: include base_animation & doors
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        Vec::with_capacity(0)
    }

    fn heat_buffer_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        Vec::with_capacity(0)
    }
}
