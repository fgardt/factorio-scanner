use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{helper, EntityWithOwnerPrototype};
use crate::data_raw::types::*;

/// [`Prototypes/CraftingMachinePrototype`](https://lua-api.factorio.com/latest/prototypes/CraftingMachinePrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct CraftingMachinePrototype<T: super::Renderable>(
    EntityWithOwnerPrototype<CraftingMachineData<T>>,
);

impl<T: super::Renderable> super::Renderable for CraftingMachinePrototype<T> {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        self.0.render(options)
    }
}

/// [`Prototypes/CraftingMachinePrototype`](https://lua-api.factorio.com/latest/prototypes/CraftingMachinePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct CraftingMachineData<T: super::Renderable> {
    pub energy_usage: Energy,
    pub crafting_speed: f64,
    pub crafting_categories: Vec<RecipeCategoryID>,
    pub energy_source: AnyEnergySource,

    pub fluid_boxes: Option<CraftingMachineFluidBoxHell>, // THIS IS HORROR
    pub allowed_effects: Option<EffectTypeLimitation>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub scale_entity_info_icon: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub show_recipe_icon: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub return_ingredients_on_change: bool,

    pub animation: Option<Animation4Way>,
    pub idle_animation: Option<Animation4Way>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub always_draw_idle_animation: bool,

    pub default_recipe_tint: Option<Color>,
    pub shift_animation_waypoints: Option<ShiftAnimationWaypoints>,

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_u16",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub shift_animation_waypoint_stop_duration: u16,

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_u16",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub shift_animation_transition_duration: u16,

    pub status_colors: Option<StatusColors>,
    pub entity_info_icon_shift: Option<Vector>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_entity_info_icon_background: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub match_animation_speed_to_activity: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub show_recipe_icon_on_map: bool,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub base_productivity: f64,

    pub module_specification: Option<ModuleSpecification>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub working_visualisations: Vec<WorkingVisualisation>,

    #[serde(flatten)]
    pub child: T,
}

impl<T: super::Renderable> super::Renderable for CraftingMachineData<T> {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        if self.always_draw_idle_animation && self.idle_animation.is_some() {
            self.idle_animation.as_ref()?
        } else {
            self.animation.as_ref()?
        }
        .render(options.factorio_dir, &options.used_mods, &options.into())
    }
}

// TODO: find a better way to work around this abomination of a type
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CraftingMachineFluidBoxHell {
    Array(Vec<FluidBox>),
    WHY(HashMap<String, CraftingMachineFluidBoxCursedType>),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CraftingMachineFluidBoxCursedType {
    FluidBox(FluidBox),
    OffWhenNoFluidRecipe(bool),
}

/// [`Prototypes/FurnacePrototype`](https://lua-api.factorio.com/latest/prototypes/FurnacePrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct FurnacePrototype(CraftingMachinePrototype<FurnaceData>);

impl super::Renderable for FurnacePrototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        self.0.render(options)
    }
}

/// [`Prototypes/FurnacePrototype`](https://lua-api.factorio.com/latest/prototypes/FurnacePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct FurnaceData {
    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub result_inventory_size: ItemStackIndex,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub source_inventory_size: ItemStackIndex,

    pub cant_insert_at_source_message_key: Option<String>,
    // TODO: `entity_info_icon_shift` has overriden default
}

impl super::Renderable for FurnaceData {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Prototypes/AssemblingMachinePrototype`](https://lua-api.factorio.com/latest/prototypes/AssemblingMachinePrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct AssemblingMachinePrototype(CraftingMachinePrototype<AssemblingMachineData>);

impl super::Renderable for AssemblingMachinePrototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        self.0.render(options)
    }
}

/// [`Prototypes/AssemblingMachinePrototype`](https://lua-api.factorio.com/latest/prototypes/AssemblingMachinePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct AssemblingMachineData {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fixed_recipe: RecipeID,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub gui_title_key: String,

    #[serde(
        default = "helper::u8_max",
        skip_serializing_if = "helper::is_max_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub ingredient_count: u8,
    // TODO: `entity_info_icon_shift` has overriden default
}

impl super::Renderable for AssemblingMachineData {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Prototypes/RocketSiloPrototype`](https://lua-api.factorio.com/latest/prototypes/RocketSiloPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct RocketSiloPrototype(CraftingMachinePrototype<RocketSiloData>);

impl super::Renderable for RocketSiloPrototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        self.0.render(options)
    }
}

/// [`Prototypes/RocketSiloPrototype`](https://lua-api.factorio.com/latest/prototypes/RocketSiloPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct RocketSiloData {
    pub active_energy_usage: Energy,
    pub lamp_energy_usage: Energy,
    pub rocket_entity: EntityID,

    pub satellite_animation: Option<Animation>,
    pub satellite_shadow_animation: Option<Animation>,
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

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_u16",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub rocket_result_inventory_size: ItemStackIndex,

    #[serde(flatten)]
    pub assembler_data: AssemblingMachineData,
    // not implemented
    // pub alarm_trigger: Option<TriggerEffect>,
    // pub clamps_on_trigger: Option<TriggerEffect>,
    // pub clamps_off_trigger: Option<TriggerEffect>,
    // pub doors_trigger: Option<TriggerEffect>,
    // pub raise_rocket_trigger: Option<TriggerEffect>,
    // pub alarm_sound: Option<Sound>,
    // pub clamps_on_sound: Option<Sound>,
    // pub clamps_off_sound: Option<Sound>,
    // pub doors_sound: Option<Sound>,
    // pub raise_rocket_sound: Option<Sound>,
    // pub flying_sound: Option<Sound>,
}

impl super::Renderable for RocketSiloData {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}