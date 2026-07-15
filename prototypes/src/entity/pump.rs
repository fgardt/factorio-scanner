use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::{EnergyEntityData, EntityWithOwnerPrototype, FluidBoxEntityData, WireEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/PumpPrototype`](https://lua-api.factorio.com/latest/prototypes/PumpPrototype.html)
pub type PumpPrototype =
    EntityWithOwnerPrototype<FluidBoxEntityData<EnergyEntityData<WireEntityData<PumpData>>>>;

/// [`Prototypes/PumpPrototype`](https://lua-api.factorio.com/latest/prototypes/PumpPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct PumpData {
    pub energy_usage: Energy,
    pub pumping_speed: FluidAmount,
    pub animations: Animation4Way,

    #[serde(
        default = "helper::f64_1_64",
        skip_serializing_if = "helper::is_1_64_f64"
    )]
    pub fluid_wagon_connector_speed: f64,

    #[serde(
        default = "helper::f64_2_2",
        skip_serializing_if = "helper::is_2_2_f64"
    )]
    pub fluid_wagon_tank_valve_max_distance: f64,

    #[serde(
        default = "helper::u8_1",
        skip_serializing_if = "helper::is_1_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub fluid_wagon_connector_frame_count: u8,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub flow_scaling: bool,

    pub fluid_animation: Option<Animation4Way>,
    pub glass_pictures: Option<Sprite4Way>,
    pub frozen_patch: Option<Sprite4Way>,

    pub wagon_connection_graphics: Option<PumpWagonConnectionGraphics>,
    // not implemented
    // pub base_lifting_sound: Option<Sound>,
    // pub arm_orienting_sound: Option<Sound>,
    // pub clamp_sound: Option<Sound>,
}

impl super::Entity for PumpData {}

impl super::Renderable for PumpData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self.animations.render(
            render_layers.scale(),
            used_mods,
            image_cache,
            &options.into(),
        )?;

        render_layers.add_entity(res, &options.position);

        Some(())
    }
}

/// [`Types/PumpWagonConnectionGraphics`](https://lua-api.factorio.com/latest/types/PumpWagonConnectionGraphics.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct PumpWagonConnectionGraphics {
    #[serde(default = "helper::f64_05", skip_serializing_if = "helper::is_05_f64")]
    pub base_animation_finished_at_progress: f64,
    #[serde(
        default = "helper::f64_075",
        skip_serializing_if = "helper::is_075_f64"
    )]
    pub clamp_animation_starts_at_progress: f64,

    #[serde(
        default = "helper::f32_015",
        skip_serializing_if = "helper::is_015_f32"
    )]
    pub height_diff_to_wagon: f32,
    #[serde(
        default = "helper::f32_n005",
        skip_serializing_if = "helper::is_n005_f32"
    )]
    pub part2_crop_adjustment: f32,
    #[serde(
        default = "helper::f32_n005",
        skip_serializing_if = "helper::is_n005_f32"
    )]
    pub part2_shadow_crop_adjustment: f32,
    #[serde(
        default = "helper::f32_n0375",
        skip_serializing_if = "helper::is_n0375_f32"
    )]
    pub clamp_y_shift: f32,

    pub base: Option<BasePumpWagonConnectionAnimations>,
    pub part_1: Option<RotatedSprite>,
    pub part_1_shadow: Option<RotatedSprite>,
    pub part_2: Option<RotatedSprite>,
    pub part_2_shadow: Option<RotatedSprite>,
    pub suction_clamp: Option<Animation>,
    pub suction_clamp_shadow: Option<Animation>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub part1_to_2_shift: Vector,

    pub top_pivot_shift: Option<PumpWagonConnectionShift4Way>,
    pub resting_position_shift: Option<PumpWagonConnectionShift4Way>,

    pub shadow_shift: Option<Vector>,
}

/// [`Types/BasePumpWagonConnectionAnimations`](https://lua-api.factorio.com/latest/types/BasePumpWagonConnectionAnimations.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct BasePumpWagonConnectionAnimations {
    pub input: Option<BasePumpWagonConnectionAnimations4Way>,
    pub output: Option<BasePumpWagonConnectionAnimations4Way>,
}

/// [`Types/BasePumpWagonConnectionAnimations4Way`](https://lua-api.factorio.com/latest/types/BasePumpWagonConnectionAnimations4Way.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct BasePumpWagonConnectionAnimations4Way {
    pub north: Animation,
    pub west: Animation,
    pub south: Animation,
    pub east: Animation,
}

/// [`Types/PumpWagonConnectionShift4Way`](https://lua-api.factorio.com/latest/types/PumpWagonConnectionShift4Way.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct PumpWagonConnectionShift4Way {
    pub north: Vector,
    pub west: Vector,
    pub south: Vector,
    pub east: Vector,
}
