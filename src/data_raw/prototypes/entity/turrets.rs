use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{helper, EntityWithOwnerPrototype};
use crate::data_raw::types::*;

/// [`Prototypes/TurretPrototype`](https://lua-api.factorio.com/latest/prototypes/TurretPrototype.html)
pub type TurretPrototype = EntityWithOwnerPrototype<TurretData>;

/// [`Prototypes/TurretPrototype`](https://lua-api.factorio.com/latest/prototypes/TurretPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct TurretData {
    pub folded_animation: RotatedAnimation4Way,
    pub call_for_help_radius: f64,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub shoot_in_prepare_state: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub turret_base_has_direction: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub random_animation_offset: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub secondary_animation: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub attack_from_start_frame: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub allow_turning_when_starting_attack: bool,

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub base_picture_secondary_draw_order: u8,

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub gun_animation_secondary_draw_order: u8,

    // TODO: defaults
    pub base_picture_render_layer: Option<RenderLayer>,
    pub gun_animation_render_layer: Option<RenderLayer>,

    pub base_picture: Option<Animation4Way>,
    pub preparing_animation: Option<RotatedAnimation4Way>,
    pub prepared_animation: Option<RotatedAnimation4Way>,
    pub prepared_alternative_animation: Option<RotatedAnimation4Way>,
    pub starting_attack_animation: Option<RotatedAnimation4Way>,
    pub energy_glow_animation: Option<RotatedAnimation4Way>,
    pub ending_attack_animation: Option<RotatedAnimation4Way>,
    pub folding_animation: Option<RotatedAnimation4Way>,
    pub integration: Option<Sprite>,

    // docs specify single precision float for all of these except `prepare_range`
    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub glow_light_intensity: f64,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub energy_glow_animation_flicker_strength: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub rotation_speed: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub preparing_speed: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub folded_speed: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub folded_speed_secondary: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub prepared_speed: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub prepared_speed_secondary: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub prepared_alternative_speed: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub prepared_alternative_speed_secondary: f64,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub prepared_alternative_chance: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub starting_attack_speed: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub attacking_speed: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub ending_attack_speed: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub folding_speed: f64,

    // defaults to range defined in `attack_parameters`
    pub prepare_range: Option<f64>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub alert_when_attacking: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub spawn_decorations_on_expansion: bool,
    // TODO: overridden `corpse` & `is_military_target`

    // not implemented
    // pub attack_parameters: AttackParameters,
    // pub attack_target_mask: Option<TriggerTargetMask>,
    // pub ignore_target_mask: Option<TriggerTargetMask>,
    // pub start_attacking_sound: Option<Sound>,
    // pub dying_sound: Option<Sound>,
    // pub preparing_sound: Option<Sound>,
    // pub folding_sound: Option<Sound>,
    // pub prepared_sound: Option<Sound>,
    // pub prepared_alternative_sound: Option<Sound>,
    // pub spawn_decoration: Option<CreateDecorativesTriggerEffectItem or array of that>,
}

/// [`Prototypes/AmmoTurretPrototype`](https://lua-api.factorio.com/latest/prototypes/AmmoTurretPrototype.html)
pub type AmmoTurretPrototype = EntityWithOwnerPrototype<AmmoTurretData>;

/// [`Prototypes/AmmoTurretPrototype`](https://lua-api.factorio.com/latest/prototypes/AmmoTurretPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct AmmoTurretData {
    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub inventory_size: ItemStackIndex,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub automated_ammo_count: ItemCountType,

    pub entity_info_icon_shift: Option<Vector>,

    #[serde(flatten)]
    pub parent: TurretData,
}

/// [`Prototypes/ElectricTurretPrototype`](https://lua-api.factorio.com/latest/prototypes/ElectricTurretPrototype.html)
pub type ElectricTurretPrototype = EntityWithOwnerPrototype<ElectricTurretData>;

/// [`Prototypes/ElectricTurretPrototype`](https://lua-api.factorio.com/latest/prototypes/ElectricTurretPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ElectricTurretData {
    pub energy_source: AnyEnergySource, // electric or void

    #[serde(flatten)]
    pub parent: TurretData,
}

/// [`Prototypes/FluidTurretPrototype`](https://lua-api.factorio.com/latest/prototypes/FluidTurretPrototype.html)
pub type FluidTurretPrototype = EntityWithOwnerPrototype<FluidTurretData>;

/// [`Prototypes/FluidTurretPrototype`](https://lua-api.factorio.com/latest/prototypes/FluidTurretPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct FluidTurretData {
    pub fluid_buffer_size: f64,       // docs specify single precision float
    pub fluid_buffer_input_flow: f64, // docs specify single precision float
    pub activation_buffer_ratio: f64, // docs specify single precision float
    pub fluid_box: FluidBox,

    pub muzzle_light: Option<LightDefinition>,
    pub enough_fuel_indicator_light: Option<LightDefinition>,
    pub not_enough_fuel_indicator_light: Option<LightDefinition>,
    pub muzzle_animation: Option<Animation>,
    pub folded_muzzle_animation_shift: Option<AnimatedVector>,
}

/// [`Types/AnimatedVector`](https://lua-api.factorio.com/latest/types/AnimatedVector.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimatedVector {
    pub rotations: Vec<VectorRotation>,

    pub render_layer: Option<RenderLayer>,
    pub direction_shift: Option<DirectionShift>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct VectorRotation {
    pub frames: Vec<Vector>,
    pub render_layer: Option<RenderLayer>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct DirectionShift {
    pub north: Option<Vector>,
    pub east: Option<Vector>,
    pub south: Option<Vector>,
    pub west: Option<Vector>,
}
