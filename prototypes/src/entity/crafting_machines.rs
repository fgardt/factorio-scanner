use std::ops::Deref;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::{EnergyEntityData, EntityWithOwnerPrototype, WireEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/CraftingMachinePrototype`](https://lua-api.factorio.com/latest/prototypes/CraftingMachinePrototype.html)
pub type CraftingMachinePrototype<T> =
    EntityWithOwnerPrototype<EnergyEntityData<CraftingMachineData<T>>>;

/// [`Prototypes/CraftingMachinePrototype`](https://lua-api.factorio.com/latest/prototypes/CraftingMachinePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct CraftingMachineData<T: super::Renderable> {
    pub energy_usage: Energy,
    pub crafting_speed: f64,
    pub crafting_categories: FactorioArray<RecipeCategoryID>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fluid_boxes: FactorioArray<FluidBox>,

    pub effect_receiver: Option<EffectReceiver>,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub module_slots: ItemStackIndex,
    pub allowed_effects: Option<EffectTypeLimitation>,
    pub allowed_module_categories: Option<FactorioArray<ModuleCategoryID>>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub show_recipe_icon: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub return_ingredients_on_change: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_entity_info_icon_background: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub match_animation_speed_to_activity: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub show_recipe_icon_on_map: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub fast_transfer_modules_into_module_slots_only: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub ignore_output_full: bool,

    pub graphics_set: Option<CraftingMachineGraphicsSet>,
    pub graphics_set_flipped: Option<CraftingMachineGraphicsSet>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub perceived_performance: PerceivedPerformance,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub production_health_effect: ProductionHealthEffect,

    pub trash_inventory_size: Option<ItemStackIndex>,
    pub vector_to_place_result: Option<Vector>,
    pub forced_symmetry: Option<Mirroring>,

    #[serde(flatten)]
    child: T,
}

impl<T: super::Renderable> Deref for CraftingMachineData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T: super::Renderable> super::Renderable for CraftingMachineData<T> {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        if let Some(gs) = if options.mirrored {
            self.graphics_set_flipped
                .as_ref()
                .or(self.graphics_set.as_ref())
        } else {
            self.graphics_set.as_ref()
        } {
            let anim = if gs.always_draw_idle_animation {
                gs.idle_animation.as_ref()
            } else {
                None
            }
            .or(gs.animation.as_ref())
            .and_then(|anim| {
                anim.render(
                    render_layers.scale(),
                    used_mods,
                    image_cache,
                    &options.into(),
                )
            });

            if let Some(anim_res) = anim {
                render_layers.add_entity(anim_res, &options.position);
            }
        }

        self.child
            .render(options, used_mods, render_layers, image_cache)
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<MapPosition> {
        let mut res = self.child.fluid_box_connections(options);
        res.extend(
            self.fluid_boxes
                .iter()
                .flat_map(|fb| fb.connection_points(options.direction, options.mirrored)),
        );

        res
    }

    fn heat_buffer_connections(&self, options: &super::RenderOpts) -> Vec<MapPosition> {
        self.child.heat_buffer_connections(options)
    }

    fn recipe_visible(&self) -> bool {
        self.show_recipe_icon
    }

    fn render_debug(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
    ) {
        for fb in &self.fluid_boxes {
            fb.render_debug(options, used_mods, render_layers);
        }
    }
}

/// [`Prototypes/FurnacePrototype`](https://lua-api.factorio.com/latest/prototypes/FurnacePrototype.html)
pub type FurnacePrototype = CraftingMachinePrototype<WireEntityData<FurnaceData>>;

/// [`Prototypes/FurnacePrototype`](https://lua-api.factorio.com/latest/prototypes/FurnacePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct FurnaceData {
    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub result_inventory_size: ItemStackIndex,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub source_inventory_size: ItemStackIndex,

    pub cant_insert_at_source_message_key: Option<String>,
    pub custom_input_slot_tooltip_key: Option<String>,

    pub default_recipe_finished_signal: Option<SignalIDConnector>,
    pub default_working_signal: Option<SignalIDConnector>,
}

impl super::Renderable for FurnaceData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        None
    }
}

/// [`Prototypes/AssemblingMachinePrototype`](https://lua-api.factorio.com/latest/prototypes/AssemblingMachinePrototype.html)
pub type AssemblingMachinePrototype =
    CraftingMachinePrototype<WireEntityData<AssemblingMachineData>>;

/// [`Prototypes/AssemblingMachinePrototype`](https://lua-api.factorio.com/latest/prototypes/AssemblingMachinePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct AssemblingMachineData {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fixed_recipe: RecipeID,

    pub fixed_quality: Option<QualityID>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub gui_title_key: String,

    pub default_recipe_finished_signal: Option<SignalIDConnector>,
    pub default_working_signal: Option<SignalIDConnector>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub enable_logistic_control_behavior: bool,

    #[serde(
        default = "helper::u8_max",
        skip_serializing_if = "helper::is_max_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub ingredient_count: u8,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub fluid_boxes_off_when_no_fluid_recipe: bool,

    pub disabled_when_recipe_not_researched: Option<bool>,
}

impl super::Renderable for AssemblingMachineData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        None
    }
}

/// [`Prototypes/RocketSiloPrototype`](https://lua-api.factorio.com/latest/prototypes/RocketSiloPrototype.html)
pub type RocketSiloPrototype = CraftingMachinePrototype<RocketSiloData>;

/// [`Prototypes/RocketSiloPrototype`](https://lua-api.factorio.com/latest/prototypes/RocketSiloPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct RocketSiloData {
    pub active_energy_usage: Energy,
    pub lamp_energy_usage: Energy,
    pub rocket_entity: EntityID,

    pub arm_01_back_animation: Animation,
    pub arm_02_right_animation: Animation,
    pub arm_03_front_animation: Animation,
    pub shadow_sprite: Sprite,
    pub hole_sprite: Sprite,
    pub hole_light_sprite: Sprite,
    pub rocket_shadow_overlay_sprite: Sprite,
    pub rocket_glow_overlay_sprite: Sprite,
    pub door_back_sprite: Sprite,
    pub door_front_sprite: Sprite,
    pub base_day_sprite: Sprite,
    pub base_front_sprite: Sprite,
    pub red_lights_back_sprites: Sprite,
    pub red_lights_front_sprites: Sprite,

    pub base_frozen: Option<Sprite>,
    pub base_front_frozen: Option<Sprite>,
    pub hole_frozen: Option<Sprite>,
    pub door_back_frozen: Option<Sprite>,
    pub door_front_frozen: Option<Sprite>,

    pub hole_clipping_box: BoundingBox,
    pub door_back_open_offset: Vector,
    pub door_front_open_offset: Vector,
    pub silo_fade_out_start_distance: f64,
    pub silo_fade_out_end_distance: f64,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub times_to_blink: u8,
    pub light_blinking_speed: f64,
    pub door_opening_speed: f64,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub rocket_parts_required: u32,

    pub rocket_quick_relaunch_start_offset: f64,

    pub satellite_animation: Option<Animation>,
    pub satellite_shadow_animation: Option<Animation>,
    pub base_night_sprite: Option<Sprite>,
    pub base_light: Option<LightDefinition>,
    pub base_engine_light: Option<LightDefinition>,

    #[serde(
        default = "helper::u8_30",
        skip_serializing_if = "helper::is_30_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub rocket_rising_delay: u8,

    #[serde(
        default = "helper::u8_120",
        skip_serializing_if = "helper::is_120_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub launch_wait_time: u8,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub render_not_in_network_icon: bool,

    pub rocket_parts_storage_cap: Option<u32>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub to_be_inserted_to_rocket_inventory_size: ItemStackIndex,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub rocket_supply_inventory_size: ItemStackIndex,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub logistic_trash_inventory_size: ItemStackIndex,

    pub cargo_station_parameters: CargoStationParameters,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub launch_to_space_platforms: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub can_launch_without_landing_pads: bool,

    #[serde(flatten)]
    assembler_data: AssemblingMachineData,
    // not implemented
    // pub alarm_trigger: Option<TriggerEffect>,
    // pub clamps_on_trigger: Option<TriggerEffect>,
    // pub clamps_off_trigger: Option<TriggerEffect>,
    // pub doors_trigger: Option<TriggerEffect>,
    // pub raise_rocket_trigger: Option<TriggerEffect>,
    // pub alarm_sound: Option<Sound>,
    // pub quick_alarm_sound: Option<Sound>,
    // pub clamps_on_sound: Option<Sound>,
    // pub clamps_off_sound: Option<Sound>,
    // pub doors_sound: Option<Sound>,
    // pub raise_rocket_sound: Option<Sound>,
}

impl Deref for RocketSiloData {
    type Target = AssemblingMachineData;

    fn deref(&self) -> &Self::Target {
        &self.assembler_data
    }
}

impl super::Renderable for RocketSiloData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = merge_renders(
            &[
                self.door_back_sprite.render(
                    render_layers.scale(),
                    used_mods,
                    image_cache,
                    &options.into(),
                ),
                self.door_front_sprite.render(
                    render_layers.scale(),
                    used_mods,
                    image_cache,
                    &options.into(),
                ),
                self.base_day_sprite.render(
                    render_layers.scale(),
                    used_mods,
                    image_cache,
                    &options.into(),
                ),
                self.arm_01_back_animation.render(
                    render_layers.scale(),
                    used_mods,
                    image_cache,
                    &options.into(),
                ),
                self.arm_02_right_animation.render(
                    render_layers.scale(),
                    used_mods,
                    image_cache,
                    &options.into(),
                ),
                self.arm_03_front_animation.render(
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
    }
}
