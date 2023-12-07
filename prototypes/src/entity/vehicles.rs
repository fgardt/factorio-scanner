use std::ops::{Deref, Rem};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;

use super::{helper, ArtilleryTurretCannonBarrelShiftings, EntityWithOwnerPrototype};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/VehiclePrototype`](https://lua-api.factorio.com/latest/prototypes/VehiclePrototype.html)
pub type VehiclePrototype<T> = EntityWithOwnerPrototype<VehicleData<T>>;

/// [`Prototypes/VehiclePrototype`](https://lua-api.factorio.com/latest/prototypes/VehiclePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleData<T: super::Renderable> {
    pub weight: f64,

    #[serde(flatten)]
    pub breaking: BreakingVariant,

    #[serde(flatten)]
    pub friction: FrictionVariant,

    pub energy_per_hit_point: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub terrain_friction_modifier: f64, // docs say single precision float

    #[serde(
        default = "helper::f64_1_60",
        skip_serializing_if = "helper::is_1_60_f64"
    )]
    pub sound_minimum_speed: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub sound_scaling_ratio: f64,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub stop_trigger_speed: f64,

    pub equipment_grid: Option<EquipmentGridID>,
    pub minimap_representation: Option<Sprite>,
    pub selected_minimap_representation: Option<Sprite>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_passengers: bool,

    #[serde(flatten)]
    child: T,
    // not implemented
    // pub crash_trigger: Option<TriggerEffect>,
    // pub stop_trigger: Option<TriggerEffect>,
}

impl<T: super::Renderable> Deref for VehicleData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T: super::Renderable> super::Renderable for VehicleData<T> {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> crate::RenderOutput {
        self.child
            .render(options, used_mods, render_layers, image_cache)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BreakingVariant {
    Power { braking_power: Energy },
    Force { braking_force: f64 },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FrictionVariant {
    Friction { friction: f64 },
    Force { friction_force: f64 },
}

/// [`Prototypes/CarPrototype`](https://lua-api.factorio.com/latest/prototypes/CarPrototype.html)
pub type CarPrototype = VehiclePrototype<CarData>;

/// [`Prototypes/CarPrototype`](https://lua-api.factorio.com/latest/prototypes/CarPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct CarData {
    pub animation: RotatedAnimation,
    pub effectivity: f64,
    pub consumption: Energy,
    pub rotation_speed: f64,

    #[serde(flatten)]
    pub energy_source: BurnerOrVoidEnergySource,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub inventory_size: ItemStackIndex,

    pub turret_animation: Option<RotatedAnimation>,
    pub light_animation: Option<RotatedAnimation>,

    // TODO: default
    pub render_layer: Option<RenderLayer>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub tank_driving: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_belt_immunity: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub immune_to_tree_impacts: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub immune_to_rock_impacts: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub immune_to_cliff_impacts: bool,

    #[serde(
        default = "helper::f64_001",
        skip_serializing_if = "helper::is_001_f64"
    )]
    pub turret_rotation_speed: f64,

    #[serde(
        default = "helper::u32_60",
        skip_serializing_if = "helper::is_60_u32",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub turret_return_timeout: u32,

    pub light: Option<LightDefinition>,

    // docs say single precision float
    #[serde(default = "helper::f64_03", skip_serializing_if = "helper::is_03_f64")]
    pub darkness_to_render_light_animation: f64,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guns: FactorioArray<ItemID>,
    // not implemented
    // pub sound_no_fuel: Option<Sound>,
    // pub track_particle_triggers: Option<FootstepTriggerEffectList>,
}

impl super::Renderable for CarData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> crate::RenderOutput {
        None
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BurnerOrVoidEnergySource {
    Burner { burner: BurnerEnergySource },
    Other { energy_source: AnyEnergySource }, // this must be a void energy source
}

/// [`Prototypes/RollingStockPrototype`](https://lua-api.factorio.com/latest/prototypes/RollingStockPrototype.html)
pub type RollingStockPrototype<T> = VehiclePrototype<RollingStockData<T>>;

/// [`Prototypes/RollingStockPrototype`](https://lua-api.factorio.com/latest/prototypes/RollingStockPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RollingStockData<T: super::Renderable> {
    pub max_speed: f64,
    pub air_resistance: f64,
    pub joint_distance: f64,
    pub connection_distance: f64,
    pub pictures: RotatedSprite,
    pub vertical_selection_shift: f64,

    #[serde(default = "helper::f64_10", skip_serializing_if = "helper::is_10_f64")]
    pub tie_distance: f64,

    pub back_light: Option<LightDefinition>,
    pub stand_by_light: Option<LightDefinition>,
    pub wheels: Option<RotatedSprite>,
    pub horizontal_doors: Option<Animation>,
    pub vertical_doors: Option<Animation>,
    pub color: Option<Color>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_manual_color: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub allow_robot_dispatch_in_automatic_mode: bool,

    #[serde(flatten)]
    child: T,
    // not implemented
    // pub drive_over_tie_trigger: Option<TriggerEffect>,
}

impl<T: super::Renderable> Deref for RollingStockData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T: super::Renderable> super::Renderable for RollingStockData<T> {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> crate::RenderOutput {
        let mut empty = true;

        let orientation = options
            .orientation
            .unwrap_or_else(|| options.direction.to_orientation());

        let options = &super::RenderOpts {
            orientation: Some(orientation.projected_orientation()),
            ..options.clone()
        };

        if let Some(wheels) = self.wheels.as_ref() {
            let offset =
                (Direction::North.get_offset() * (self.joint_distance / 2.0)).rotate(orientation);

            let rail_offset = Vector::new(
                0.0,
                -(0.25
                    * (orientation * std::f64::consts::TAU + std::f64::consts::FRAC_PI_2)
                        .cos()
                        .abs()),
            );

            if let Some((img, shift)) = self.wheels.as_ref().and_then(|b| {
                b.render(
                    render_layers.scale(),
                    used_mods,
                    image_cache,
                    &options.into(),
                )
            }) {
                empty = false;

                render_layers.add(
                    (img, shift + offset.flip() + rail_offset),
                    &options.position,
                    crate::InternalRenderLayer::EntityHigh,
                );
            }

            let other_wheel_opts = RotatedSpriteRenderOpts {
                orientation: (orientation.projected_orientation() + 0.5).rem(1.0),
                runtime_tint: options.runtime_tint,
            };

            if let Some((img, shift)) = self.wheels.as_ref().and_then(|b| {
                b.render(
                    render_layers.scale(),
                    used_mods,
                    image_cache,
                    &other_wheel_opts,
                )
            }) {
                empty = false;

                render_layers.add(
                    (img, shift + offset + rail_offset),
                    &options.position,
                    crate::InternalRenderLayer::EntityHigh,
                );
            }
        }

        if let Some(res) = self.pictures.render(
            render_layers.scale(),
            used_mods,
            image_cache,
            &options.into(),
        ) {
            empty = false;

            render_layers.add(
                res,
                &options.position,
                crate::InternalRenderLayer::EntityHigh,
            );
        }

        let child = self
            .child
            .render(options, used_mods, render_layers, image_cache);

        if empty && child.is_none() {
            None
        } else {
            Some(())
        }
    }
}

/// [`Prototypes/ArtilleryWagonPrototype`](https://lua-api.factorio.com/latest/prototypes/ArtilleryWagonPrototype.html)
pub type ArtilleryWagonPrototype = RollingStockPrototype<ArtilleryWagonData>;

/// [`Prototypes/ArtilleryWagonPrototype`](https://lua-api.factorio.com/latest/prototypes/ArtilleryWagonPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ArtilleryWagonData {
    pub gun: ItemID,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub inventory_size: ItemStackIndex,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub ammo_stack_limit: ItemCountType,

    pub turret_rotation_speed: f64,
    pub manual_range_modifier: f64,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub disable_automatic_firing: bool,

    pub cannon_base_pictures: Option<RotatedSprite>,
    pub cannon_barrel_pictures: Option<RotatedSprite>,
    pub cannon_base_shiftings: Option<FactorioArray<Vector>>,

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_u16",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub turn_after_shooting_cooldown: u16,

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_u16",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub cannon_parking_frame_count: u16,

    // docs say single precision float
    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub cannon_parking_speed: f64,

    #[serde(flatten)]
    pub cannon_barrel_recoil_shiftings: Option<ArtilleryTurretCannonBarrelShiftings>,
    // not implemented
    // pub rotating_sound: Option<InterruptibleSound>,
    // pub rotating_stopped_sound: Option<Sound>,
}

impl super::Renderable for ArtilleryWagonData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> crate::RenderOutput {
        let mut empty = true;

        let offset = self.cannon_base_shiftings.as_ref().map_or_else(
            || Vector::new(0.0, 0.0),
            |shifts| {
                if shifts.is_empty() {
                    Vector::new(0.0, 0.0)
                } else {
                    let len_f = shifts.len() as f64;
                    let idx = (len_f
                        * options
                            .orientation
                            .unwrap_or_else(|| options.direction.to_orientation()))
                    .floor();

                    let idx = if idx < 0.0 {
                        len_f + idx.rem(len_f)
                    } else {
                        idx.rem(len_f)
                    } as usize;

                    shifts[idx]
                }
            },
        );

        if let Some((img, shift)) = self.cannon_barrel_pictures.as_ref().and_then(|b| {
            b.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        }) {
            empty = false;

            render_layers.add(
                (img, shift + offset),
                &options.position,
                crate::InternalRenderLayer::EntityHigher,
            );
        }

        if let Some((img, shift)) = self.cannon_base_pictures.as_ref().and_then(|b| {
            b.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        }) {
            empty = false;

            render_layers.add(
                (img, shift + offset),
                &options.position,
                crate::InternalRenderLayer::EntityHigher,
            );
        }

        if empty {
            None
        } else {
            Some(())
        }
    }
}

/// [`Prototypes/CargoWagonPrototype`](https://lua-api.factorio.com/latest/prototypes/CargoWagonPrototype.html)
pub type CargoWagonPrototype = RollingStockPrototype<CargoWagonData>;

/// [`Prototypes/CargoWagonPrototype`](https://lua-api.factorio.com/latest/prototypes/CargoWagonPrototype.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct CargoWagonData {
    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub inventory_size: ItemStackIndex,
}

impl super::Renderable for CargoWagonData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> crate::RenderOutput {
        Some(())
    }
}

/// [`Prototypes/FluidWagonPrototype`](https://lua-api.factorio.com/latest/prototypes/FluidWagonPrototype.html)
pub type FluidWagonPrototype = RollingStockPrototype<FluidWagonData>;

/// [`Prototypes/FluidWagonPrototype`](https://lua-api.factorio.com/latest/prototypes/FluidWagonPrototype.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct FluidWagonData {
    pub capacity: f64,

    // TODO: skip serializing if default
    #[serde(default)]
    pub tank_count: FluidWagonTankCount,
}

impl super::Renderable for FluidWagonData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> crate::RenderOutput {
        Some(())
    }
}

#[derive(Debug, Default, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum FluidWagonTankCount {
    Single = 1,
    Double = 2,
    #[default]
    Triple = 3,
}

/// [`Prototypes/LocomotivePrototype`](https://lua-api.factorio.com/latest/prototypes/LocomotivePrototype.html)
pub type LocomotivePrototype = RollingStockPrototype<LocomotiveData>;

/// [`Prototypes/LocomotivePrototype`](https://lua-api.factorio.com/latest/prototypes/LocomotivePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct LocomotiveData {
    pub max_power: Energy,
    pub reversing_power_modifier: f64,

    #[serde(flatten)]
    pub energy_source: BurnerOrVoidEnergySource,

    pub front_light: Option<LightDefinition>,
    pub front_light_pictures: Option<RotatedSprite>,

    // docs say single precision float
    #[serde(default = "helper::f64_03", skip_serializing_if = "helper::is_03_f64")]
    pub darkness_to_render_light_animation: f64,

    // docs say single precision float
    #[serde(default = "helper::f64_3", skip_serializing_if = "helper::is_3_f64")]
    pub max_snap_to_train_stop_distance: f64,
}

impl super::Renderable for LocomotiveData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> crate::RenderOutput {
        Some(())
    }
}
