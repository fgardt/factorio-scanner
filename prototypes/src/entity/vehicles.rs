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

    #[serde(default = "helper::f32_1", skip_serializing_if = "helper::is_1_f32")]
    pub terrain_friction_modifier: f32,

    #[serde(default = "helper::f64_5", skip_serializing_if = "helper::is_5_f64")]
    pub impact_speed_to_volume_ratio: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub stop_trigger_speed: f64,

    pub equipment_grid: Option<EquipmentGridID>,
    pub minimap_representation: Option<Sprite>,
    pub selected_minimap_representation: Option<Sprite>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_passengers: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub deliver_category: String,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub chunk_exploration_radius: u32,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub allow_remote_driving: bool,

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
    ) -> super::RenderOutput {
        self.child
            .render(options, used_mods, render_layers, image_cache)
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        self.child.fluid_box_connections(options)
    }

    fn heat_buffer_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        self.child.heat_buffer_connections(options)
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
    pub animation: Option<RotatedAnimation>,
    pub effectivity: f64,
    pub consumption: Energy,
    pub rotation_speed: f64,

    #[serde(flatten)]
    pub energy_source: BurnerOrVoidEnergySource,

    pub turret_animation: Option<RotatedAnimation>,
    pub light_animation: Option<RotatedAnimation>,

    // TODO: default
    pub render_layer: Option<RenderLayer>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub tank_driving: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub auto_sort_inventory: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub has_belt_immunity: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub immune_to_tree_impacts: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub immune_to_rock_impacts: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub immune_to_cliff_impacts: bool,

    #[serde(
        default = "helper::f32_001",
        skip_serializing_if = "helper::is_001_f32"
    )]
    pub turret_rotation_speed: f32,

    #[serde(
        default = "helper::u32_60",
        skip_serializing_if = "helper::is_60_u32",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub turret_return_timeout: u32,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub inventory_size: ItemStackIndex,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub trash_inventory_size: ItemStackIndex,

    pub light: Option<LightDefinition>,

    #[serde(default = "helper::f32_03", skip_serializing_if = "helper::is_03_f32")]
    pub darkness_to_render_light_animation: f32,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guns: FactorioArray<ItemID>,
    // not implemented
    // pub sound_no_fuel: Option<Sound>,
    // pub track_particle_triggers: Option<FootstepTriggerEffectList>,
}

impl super::Renderable for CarData {
    fn render(
        &self,
        opts: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self.animation.as_ref()?.render(
            render_layers.scale(),
            used_mods,
            image_cache,
            &opts.into(),
        )?;

        render_layers.add_entity(res, &opts.position);

        Some(())
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
    pub pictures: Option<RollingStockRotatedSlopedGraphics>,
    pub wheels: Option<RollingStockRotatedSlopedGraphics>,
    pub vertical_selection_shift: f64,

    #[serde(default = "helper::f64_10", skip_serializing_if = "helper::is_10_f64")]
    pub tie_distance: f64,

    pub back_light: Option<LightDefinition>,
    pub stand_by_light: Option<LightDefinition>,
    pub horizontal_doors: Option<Animation>,
    pub vertical_doors: Option<Animation>,
    pub color: Option<Color>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_manual_color: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub allow_robot_dispatch_in_automatic_mode: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub default_copy_color_from_train_stop: bool,

    pub transition_collision_mask: Option<CollisionMaskConnector>,
    pub elevated_collision_mask: Option<CollisionMaskConnector>,

    #[serde(default = "helper::u8_56", skip_serializing_if = "helper::is_56_u8")]
    pub elevated_selection_priority: u8,

    #[serde(flatten)]
    child: T,
    // not implemented
    // pub drive_over_tie_trigger: Option<TriggerEffect>,
    // pub drive_over_tie_trigger_minimal_speed: f64,
    // pub elevated_rail_sound: Option<MainSound>,
    // pub drive_over_elevated_tie_trigger: Option<TriggerEffect>,
    // pub door_opening_sound: Option<InterruptibleSound>,
    // pub door_closing_sound: Option<InterruptibleSound>,
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
        opts: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let mut empty = true;

        let orientation = opts
            .orientation
            .unwrap_or_else(|| opts.direction.to_orientation());

        let opts = &super::RenderOpts {
            orientation: Some(orientation.projected_orientation()),
            ..opts.clone()
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

            if let Some((img, shift)) =
                wheels
                    .rotated
                    .render(render_layers.scale(), used_mods, image_cache, &opts.into())
            {
                empty = false;

                render_layers.add(
                    (img, shift + offset.flip() + rail_offset),
                    &opts.position,
                    crate::InternalRenderLayer::EntityHigh,
                );
            }

            let other_wheel_opts = RotatedRenderOpts::new(
                (orientation.projected_orientation() + 0.5).rem(1.0),
                opts.into(),
            );

            if let Some((img, shift)) = wheels.rotated.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &other_wheel_opts,
            ) {
                empty = false;

                render_layers.add(
                    (img, shift + offset + rail_offset),
                    &opts.position,
                    crate::InternalRenderLayer::EntityHigh,
                );
            }
        }

        if let Some(res) = self.pictures.as_ref().and_then(|p| {
            p.rotated
                .render(render_layers.scale(), used_mods, image_cache, &opts.into())
        }) {
            empty = false;

            render_layers.add(res, &opts.position, crate::InternalRenderLayer::EntityHigh);
        }

        let child = self
            .child
            .render(opts, used_mods, render_layers, image_cache);

        if empty && child.is_none() {
            None
        } else {
            Some(())
        }
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        self.child.fluid_box_connections(options)
    }

    fn heat_buffer_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        self.child.heat_buffer_connections(options)
    }
}

/// [`Types/RollingStockRotatedSlopedGraphics`](https://lua-api.factorio.com/latest/types/RollingStockRotatedSlopedGraphics.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RollingStockRotatedSlopedGraphics {
    pub rotated: RotatedSprite,
    pub sloped: Option<RotatedSprite>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub slope_back_equals_front: bool,
    #[serde(
        default = "helper::f64_1_333",
        skip_serializing_if = "helper::is_1_333_f64"
    )]
    pub slope_angle_between_frames: f64,
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

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "helper::truncating_opt_deserializer"
    )]
    pub automated_ammo_count: Option<ItemCountType>,

    pub turret_rotation_speed: f64,
    pub manual_range_modifier: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub disable_automatic_firing: bool,

    pub cannon_base_pictures: Option<RollingStockRotatedSlopedGraphics>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub cannon_base_height: f64,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub cannon_base_shift_when_vertical: f64,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub cannon_base_shift_when_horizontal: f64,

    pub cannon_barrel_pictures: Option<RollingStockRotatedSlopedGraphics>,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub turn_after_shooting_cooldown: u16,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub cannon_parking_frame_count: u16,

    #[serde(default = "helper::f32_1", skip_serializing_if = "helper::is_1_f32")]
    pub cannon_parking_speed: f32,

    #[serde(flatten)]
    pub cannon_barrel_recoil_shiftings: Option<ArtilleryTurretCannonBarrelShiftings>,
    // not implemented
    // pub rotating_sound: Option<InterruptibleSound>,
}

impl super::Renderable for ArtilleryWagonData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let mut empty = true;

        let offset = Vector::default();
        // let offset = self.cannon_base_shiftings.as_ref().map_or_else(
        //     || Vector::new(0.0, 0.0),
        //     |shifts| {
        //         if shifts.is_empty() {
        //             Vector::new(0.0, 0.0)
        //         } else {
        //             let len_f = shifts.len() as f64;
        //             let idx = (len_f
        //                 * options
        //                     .orientation
        //                     .unwrap_or_else(|| options.direction.to_orientation()))
        //             .floor();
        //             let idx = if idx < 0.0 {
        //                 len_f + idx.rem(len_f)
        //             } else {
        //                 idx.rem(len_f)
        //             } as usize;
        //             shifts[idx]
        //         }
        //     },
        // );

        if let Some((img, shift)) = self.cannon_barrel_pictures.as_ref().and_then(|b| {
            b.rotated.render(
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
            b.rotated.render(
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
    ) -> super::RenderOutput {
        Some(())
    }
}

/// [`Prototypes/FluidWagonPrototype`](https://lua-api.factorio.com/latest/prototypes/FluidWagonPrototype.html)
pub type FluidWagonPrototype = RollingStockPrototype<FluidWagonData>;

/// [`Prototypes/FluidWagonPrototype`](https://lua-api.factorio.com/latest/prototypes/FluidWagonPrototype.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct FluidWagonData {
    pub capacity: FluidAmount,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub tank_count: FluidWagonTankCount,
}

impl super::Renderable for FluidWagonData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        Some(())
    }
}

#[derive(Debug, Default, Serialize_repr, Deserialize_repr, PartialEq, Eq)]
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
    pub front_light_pictures: Option<RollingStockRotatedSlopedGraphics>,

    #[serde(default = "helper::f32_03", skip_serializing_if = "helper::is_03_f32")]
    pub darkness_to_render_light_animation: f32,

    #[serde(default = "helper::f32_3", skip_serializing_if = "helper::is_3_f32")]
    pub max_snap_to_train_stop_distance: f32,
}

impl super::Renderable for LocomotiveData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        Some(())
    }
}
