use mod_util::UsedMods;
use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_with::skip_serializing_none;

use super::{
    AnimationElement, AnimationRenderOpts, GraphicsOutput, RenderLayer, RenderableGraphics,
    SpriteVariations, VariationRenderOpts, merge_layers, merge_renders,
};
use crate::{
    Color, FactorioArray, ImageCache, LightDefinition, ModuleTint, ModuleTintMode,
    WaterReflectionDefinition,
};

/// [`Types/BeaconModuleVisualization`](https://lua-api.factorio.com/latest/types/BeaconModuleVisualization.html)
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BeaconModuleVisualization {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub has_empty_slot: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub draw_as_light: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_as_sprite: bool,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub secondary_draw_order: i8,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub apply_module_tint: ModuleTint,

    pub render_layer: Option<RenderLayer>,
    pub pictures: Option<SpriteVariations>,
}

/// [`Types/BeaconModuleVisualizations`](https://lua-api.factorio.com/latest/types/BeaconModuleVisualizations.html)
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BeaconModuleVisualizations {
    pub art_style: String,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub use_for_empty_slots: bool,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub tier_offset: i32,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub slots: FactorioArray<FactorioArray<BeaconModuleVisualization>>,
}

impl RenderableGraphics for BeaconModuleVisualizations {
    type RenderOpts = ();

    fn render(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        (): &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        merge_renders(
            &self
                .slots
                .iter()
                .flat_map(|slot| {
                    slot.iter()
                        .filter_map(|l| {
                            if l.has_empty_slot && l.draw_as_sprite {
                                l.pictures.as_ref().map(|p| {
                                    p.render(
                                        scale,
                                        used_mods,
                                        image_cache,
                                        &VariationRenderOpts::default(),
                                    )
                                })
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
            scale,
        )
    }
}

/// [`Types/BeaconGraphicsSet`](https://lua-api.factorio.com/latest/types/BeaconGraphicsSet.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct BeaconGraphicsSet {
    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_animation_when_idle: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub draw_light_when_idle: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub random_animation_offset: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub module_icons_suppressed: bool,

    pub base_layer: Option<RenderLayer>,
    pub animation_layer: Option<RenderLayer>,
    pub top_layer: Option<RenderLayer>,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub animation_progress: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub min_animation_progress: f64,

    #[serde(
        default = "helper::f64_1000",
        skip_serializing_if = "helper::is_1000_f64"
    )]
    pub max_animation_progress: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub apply_module_tint: ModuleTint,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub apply_module_tint_to_light: ModuleTint,

    pub no_modules_tint: Option<Color>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub animation_list: FactorioArray<AnimationElement>,

    pub light: Option<LightDefinition>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub module_visualisations: FactorioArray<BeaconModuleVisualizations>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub module_tint_mode: ModuleTintMode,

    pub water_reflection: Option<WaterReflectionDefinition>,
}

impl RenderableGraphics for BeaconGraphicsSet {
    type RenderOpts = AnimationRenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        // TODO: render module visualisations
        let base = merge_layers(&self.animation_list, scale, used_mods, image_cache, opts)?;

        let mut renders = Vec::new();
        renders.push(Some(base));
        self.module_visualisations
            .iter()
            .for_each(|mv| renders.push(mv.render(scale, used_mods, image_cache, &())));

        merge_renders(&renders, scale)
    }
}
