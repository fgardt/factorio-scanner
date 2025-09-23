use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::{EntityWithOwnerPrototype, WireEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/AsteroidCollectorPrototype`](https://lua-api.factorio.com/latest/prototypes/AsteroidCollectorPrototype.html)
pub type AsteroidCollectorPrototype =
    EntityWithOwnerPrototype<WireEntityData<AsteroidCollectorData>>;

/// [`Prototypes/AsteroidCollectorPrototype`](https://lua-api.factorio.com/latest/prototypes/AsteroidCollectorPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct AsteroidCollectorData {
    #[serde(default = "helper::u16_5", skip_serializing_if = "helper::is_5_u16")]
    pub arm_inventory_size: ItemStackIndex,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub arm_inventory_size_quality_increase: ItemStackIndex,

    #[serde(default = "helper::u16_39", skip_serializing_if = "helper::is_39_u16")]
    pub inventory_size: ItemStackIndex,

    #[serde(default = "helper::u16_5", skip_serializing_if = "helper::is_5_u16")]
    pub inventory_size_quality_increase: ItemStackIndex,

    pub graphics_set: AsteroidCollectorGraphicsSet,

    pub passive_energy_usage: Energy,
    pub arm_energy_usage: Energy,
    pub arm_slow_energy_usage: Energy,

    #[serde(default = "helper::f32_01", skip_serializing_if = "helper::is_01_f32")]
    pub energy_usage_quality_scaling: f32,
    #[serde(default = "helper::u32_3", skip_serializing_if = "helper::is_3_u32")]
    pub arm_count_base: u32,
    #[serde(default = "helper::u32_1", skip_serializing_if = "helper::is_1_u32")]
    pub arm_count_quality_scaling: u32,
    #[serde(default = "helper::f32_06", skip_serializing_if = "helper::is_06_f32")]
    pub head_collection_radius: f32,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub collection_box_offset: f32,
    #[serde(
        default = "helper::f32_1_5",
        skip_serializing_if = "helper::is_1_5_f32"
    )]
    pub deposit_radius: f32,
    #[serde(default = "helper::f32_01", skip_serializing_if = "helper::is_01_f32")]
    pub arm_speed_base: f32,
    #[serde(default = "helper::f32_01", skip_serializing_if = "helper::is_01_f32")]
    pub arm_speed_quality_scaling: f32,
    #[serde(default = "helper::f32_01", skip_serializing_if = "helper::is_01_f32")]
    pub arm_angular_speed_cap_base: f32,
    #[serde(default = "helper::f32_01", skip_serializing_if = "helper::is_01_f32")]
    pub arm_angular_speed_cap_quality_scaling: f32,
    #[serde(default = "helper::f32_04", skip_serializing_if = "helper::is_04_f32")]
    pub tether_size: f32,
    #[serde(default = "helper::f32_03", skip_serializing_if = "helper::is_03_f32")]
    pub unpowered_arm_speed_scale: f32,
    #[serde(default = "helper::u32_6", skip_serializing_if = "helper::is_6_u32")]
    pub minimal_arm_swing_segment_retraction: u32,
    #[serde(default = "helper::f32_01", skip_serializing_if = "helper::is_01_f32")]
    pub held_items_offset: f32,
    #[serde(
        default = "helper::f32_015",
        skip_serializing_if = "helper::is_015_f32"
    )]
    pub held_items_spread: f32,
    #[serde(default = "helper::u8_5", skip_serializing_if = "helper::is_5_u8")]
    pub held_items_display_count: u8,

    pub energy_source: AnyEnergySource,
    pub radius_visualisation_picture: Option<Sprite>,
    pub collection_radius: f64,

    #[serde(
        default = "default_arm_color_gradient",
        skip_serializing_if = "is_default_arm_color_gradient"
    )]
    pub arm_color_gradient: FactorioArray<Color>,
    // not implemented
    // pub munch_sound: Option<Sound>,
    // pub deposit_sound: Option<Sound>,
    // pub arm_extend_sound: Option<Sound>,
    // pub arm_retract_sound: Option<Sound>,
}

fn default_arm_color_gradient() -> FactorioArray<Color> {
    FactorioArray::new(vec![Color::white()])
}

fn is_default_arm_color_gradient(arm_color_gradient: &FactorioArray<Color>) -> bool {
    if arm_color_gradient.len() != 1 {
        return false;
    }

    if let Some(color) = arm_color_gradient.first() {
        return Color::is_white(color);
    }

    false
}

impl super::Renderable for AsteroidCollectorData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self.graphics_set.animation.as_ref().and_then(|a| {
            a.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        })?;

        render_layers.add_entity(res, &options.position);

        Some(())
    }
}

/// [`Prototypes/AsteroidCollectorPrototype/GraphicsSet`](https://lua-api.factorio.com/latest/prototypes/AsteroidCollectorPrototype.html#graphics_set)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct AsteroidCollectorGraphicsSet {
    pub animation: Option<Animation4Way>,
    pub status_lamp_picture_on: Option<RotatedSprite>,
    pub status_lamp_picture_full: Option<RotatedSprite>,
    pub status_lamp_picture_off: Option<RotatedSprite>,
    pub below_arm_pictures: Option<RotatedSprite>,
    pub below_ground_pictures: Option<RotatedSprite>,
    pub arm_head_animation: Option<RotatedAnimation>,
    pub arm_head_top_animation: Option<RotatedAnimation>,
    pub arm_link: Option<RotatedSprite>,
    pub water_reflection: Option<WaterReflectionDefinition>,
}
