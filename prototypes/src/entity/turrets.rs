use std::ops::Deref;

use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_with::skip_serializing_none;

use super::{EntityWithOwnerPrototype, FluidBoxEntityData, WireEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/TurretPrototype`](https://lua-api.factorio.com/latest/prototypes/TurretPrototype.html)
pub type TurretPrototype = EntityWithOwnerPrototype<WireEntityData<TurretData>>;

/// [`Prototypes/TurretPrototype`](https://lua-api.factorio.com/latest/prototypes/TurretPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct TurretData {
    pub attack_parameters: Box<AttackParameters>,

    pub folded_animation: RotatedAnimation8Way,
    pub call_for_help_radius: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub shoot_in_prepare_state: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub start_attacking_only_when_can_shoot: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub turret_base_has_direction: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub random_animation_offset: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub attack_from_start_frame: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub allow_turning_when_starting_attack: bool,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub base_picture_secondary_draw_order: u8,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub gun_animation_secondary_draw_order: u8,

    // TODO: defaults
    pub base_picture_render_layer: Option<RenderLayer>,
    pub gun_animation_render_layer: Option<RenderLayer>,

    pub graphics_set: TurretGraphicsSet,
    pub preparing_animation: Option<Box<RotatedAnimation8Way>>,
    pub prepared_animation: Option<Box<RotatedAnimation8Way>>,
    pub prepared_alternative_animation: Option<Box<RotatedAnimation8Way>>,
    pub starting_attack_animation: Option<Box<RotatedAnimation8Way>>,
    pub attacking_animation: Option<Box<RotatedAnimation8Way>>,
    pub energy_glow_animation: Option<Box<RotatedAnimation8Way>>,
    pub resource_indicator_animation: Option<Box<RotatedAnimation8Way>>,
    pub ending_attack_animation: Option<Box<RotatedAnimation8Way>>,
    pub folding_animation: Option<Box<RotatedAnimation8Way>>,
    pub integration: Option<Box<Sprite>>,

    // TODO: move non graphic values to a separate struct
    // docs specify single precision float for all of these except `prepare_range`
    // #[serde(default, skip_serializing_if = "helper::is_default")]
    // pub glow_light_intensity: f64,
    // #[serde(default, skip_serializing_if = "helper::is_default")]
    // pub energy_glow_animation_flicker_strength: f64,
    // #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    // pub rotation_speed: f64,
    // #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    // pub preparing_speed: f64,
    // #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    // pub folded_speed: f64,
    // #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    // pub folded_speed_secondary: f64,
    // #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    // pub prepared_speed: f64,
    // #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    // pub prepared_speed_secondary: f64,
    // #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    // pub prepared_alternative_speed: f64,
    // #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    // pub prepared_alternative_speed_secondary: f64,
    // #[serde(default, skip_serializing_if = "helper::is_default")]
    // pub prepared_alternative_chance: f64,
    // #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    // pub starting_attack_speed: f64,
    // #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    // pub attacking_speed: f64,
    // #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    // pub ending_attack_speed: f64,
    // #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    // pub folding_speed: f64,
    // // defaults to range defined in `attack_parameters`
    // pub prepare_range: Option<f64>,
    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub alert_when_attacking: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub spawn_decorations_on_expansion: bool,
    // TODO: overridden `corpse` & `is_military_target`

    // not implemented
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

impl super::Renderable for TurretData {
    fn render(
        &self,
        opts: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self
            .graphics_set
            .base_visualisation
            .as_ref()
            .and_then(|a| a.render(render_layers.scale(), used_mods, image_cache, opts))?;

        render_layers.add_entity(res, &opts.position);

        Some(())
    }
}

/// [`Prototypes/AmmoTurretPrototype`](https://lua-api.factorio.com/latest/prototypes/AmmoTurretPrototype.html)
pub type AmmoTurretPrototype = EntityWithOwnerPrototype<WireEntityData<AmmoTurretData>>;

/// [`Prototypes/AmmoTurretPrototype`](https://lua-api.factorio.com/latest/prototypes/AmmoTurretPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct AmmoTurretData {
    pub energy_source: Option<Box<ElectricEnergySource>>,
    pub energy_per_shot: Option<Energy>,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub inventory_size: ItemStackIndex,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub automated_ammo_count: ItemCountType,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub prepare_with_no_ammo: bool,

    #[serde(flatten)]
    parent: TurretData,
}

impl Deref for AmmoTurretData {
    type Target = TurretData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl super::Renderable for AmmoTurretData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        self.parent
            .render(options, used_mods, render_layers, image_cache)
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<(MapPosition, Direction)> {
        self.parent.fluid_box_connections(options)
    }

    fn heat_buffer_connections(
        &self,
        options: &super::RenderOpts,
    ) -> Vec<(MapPosition, Direction)> {
        self.parent.heat_buffer_connections(options)
    }
}

/// [`Prototypes/ElectricTurretPrototype`](https://lua-api.factorio.com/latest/prototypes/ElectricTurretPrototype.html)
pub type ElectricTurretPrototype = EntityWithOwnerPrototype<WireEntityData<ElectricTurretData>>;

/// [`Prototypes/ElectricTurretPrototype`](https://lua-api.factorio.com/latest/prototypes/ElectricTurretPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ElectricTurretData {
    pub energy_source: AnyEnergySource, // electric or void

    #[serde(flatten)]
    parent: TurretData,
}

impl Deref for ElectricTurretData {
    type Target = TurretData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl super::Renderable for ElectricTurretData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        self.parent
            .render(options, used_mods, render_layers, image_cache)
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<(MapPosition, Direction)> {
        self.parent.fluid_box_connections(options)
    }

    fn heat_buffer_connections(
        &self,
        options: &super::RenderOpts,
    ) -> Vec<(MapPosition, Direction)> {
        self.parent.heat_buffer_connections(options)
    }
}

/// [`Prototypes/FluidTurretPrototype`](https://lua-api.factorio.com/latest/prototypes/FluidTurretPrototype.html)
pub type FluidTurretPrototype =
    EntityWithOwnerPrototype<FluidBoxEntityData<WireEntityData<FluidTurretData>>>;

/// [`Prototypes/FluidTurretPrototype`](https://lua-api.factorio.com/latest/prototypes/FluidTurretPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct FluidTurretData {
    pub fluid_buffer_size: FluidAmount,
    pub fluid_buffer_input_flow: FluidAmount,
    pub activation_buffer_ratio: FluidAmount,

    pub muzzle_light: Option<LightDefinition>,
    pub enough_fuel_indicator_light: Option<LightDefinition>,
    pub not_enough_fuel_indicator_light: Option<LightDefinition>,
    pub muzzle_animation: Option<Animation>,
    pub folded_muzzle_animation_shift: Option<AnimatedVector>,

    // TODO: more properties (AnimatedVector, ammo/fuel sprites)
    #[serde(flatten)]
    parent: TurretData,
}

impl Deref for FluidTurretData {
    type Target = TurretData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl super::Renderable for FluidTurretData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        self.parent
            .render(options, used_mods, render_layers, image_cache)
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<(MapPosition, Direction)> {
        self.parent.fluid_box_connections(options)
    }

    fn heat_buffer_connections(
        &self,
        options: &super::RenderOpts,
    ) -> Vec<(MapPosition, Direction)> {
        self.parent.heat_buffer_connections(options)
    }
}

/// [`Types/AnimatedVector`](https://lua-api.factorio.com/latest/types/AnimatedVector.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimatedVector {
    pub rotations: FactorioArray<VectorRotation>,

    pub render_layer: Option<RenderLayer>,
    pub direction_shift: Option<DirectionShift>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct VectorRotation {
    pub frames: FactorioArray<Vector>,
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

/// [`Types/TurretGraphicsSet`](https://lua-api.factorio.com/latest/types/TurretGraphicsSet.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct TurretGraphicsSet {
    pub base_visualisation: Option<TurretBaseVisualisation>,
    pub water_reflection: Option<WaterReflectionDefinition>,
}

/// [`Types/TurretBaseVisualisation`](https://lua-api.factorio.com/latest/types/TurretBaseVisualisation.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TurretBaseVisualisation {
    Array(FactorioArray<Self>),
    Struct {
        #[serde(default = "rl_lo", skip_serializing_if = "is_rl_lo")]
        render_layer: RenderLayer,

        #[serde(default, skip_serializing_if = "helper::is_default")]
        secondary_draw_order: i8,

        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        enabled_states: FactorioArray<TurretState>,

        #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
        draw_when_has_energy: bool,
        #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
        draw_when_no_energy: bool,
        #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
        draw_when_has_ammo: bool,
        #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
        draw_when_no_ammo: bool,
        #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
        draw_when_frozen: bool,
        #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
        draw_when_not_frozen: bool,

        animation: Animation4Way,
    },
}

impl TurretBaseVisualisation {
    fn render(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &super::RenderOpts,
    ) -> Option<GraphicsOutput> {
        match self {
            Self::Array(arr) => merge_renders(
                &arr.iter()
                    .map(|l| l.render(scale, used_mods, image_cache, opts))
                    .collect::<Box<_>>(),
                scale,
            ),
            Self::Struct { animation, .. } => {
                animation.render(scale, used_mods, image_cache, &opts.into())
            }
        }
    }
}

const fn rl_lo() -> RenderLayer {
    RenderLayer::LowerObject
}

#[expect(clippy::trivially_copy_pass_by_ref)]
fn is_rl_lo(rl: &RenderLayer) -> bool {
    *rl == RenderLayer::LowerObject
}

/// [`Types/TurretState`](https://lua-api.factorio.com/latest/types/TurretState.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TurretState {
    Folded,
    Preparing,
    Prepared,
    StartingAttack,
    Attacking,
    EndingAttack,
    RotateForFolding,
    Folding,
}
