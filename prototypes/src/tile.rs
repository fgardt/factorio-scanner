use mod_util::UsedMods;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;
use types::{
    CollisionMaskConnector, Color, FactorioArray, Icon, ImageCache, MapPosition, PlaceableBy,
    RenderableGraphics, TileID, TileRenderOpts, TileSprite, TileSpriteWithProbability,
};

use crate::{helper_macro::namespace_struct, InternalRenderLayer};

/// [`Prototypes/TilePrototype`](https://lua-api.factorio.com/latest/prototypes/TilePrototype.html)
pub type TilePrototype = crate::BasePrototype<TilePrototypeData>;

impl TilePrototype {
    pub fn render(
        &self,
        position: &MapPosition,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> Option<()> {
        let opts = TileRenderOpts {
            runtime_tint: Some(self.tint),
            position: *position,
        };

        self.variants
            .material_background
            .as_ref()
            .and_then(|mb| mb.render(render_layers.scale(), used_mods, image_cache, &opts))
            .or_else(|| {
                self.variants.main.iter().next()?.render(
                    render_layers.scale(),
                    used_mods,
                    image_cache,
                    &opts,
                )
            })
            .map(|res| render_layers.add(res, position, InternalRenderLayer::Ground))
    }
}

/// [`Prototypes/TilePrototype`](https://lua-api.factorio.com/latest/prototypes/TilePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct TilePrototypeData {
    pub collision_mask: CollisionMaskConnector,
    pub layer: u8,

    pub variants: TileTransitionsVariants,
    pub map_color: Color,
    pub pollution_absorption_per_second: f64,

    #[serde(flatten)]
    pub icon: Option<Icon>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub transition_overlay_layer_offset: u8,

    pub layer_group: Option<TileRenderLayer>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub draw_in_water_layer: bool,

    pub transition_merges_with_tile: Option<TileID>,

    #[serde(default = "Color::white", skip_serializing_if = "Color::is_white")]
    pub effect_color: Color,
    #[serde(default = "Color::white", skip_serializing_if = "Color::is_white")]
    pub tint: Color,

    // pub walking_sound: Option<Sound>,
    // pub build_sound: Option<Sound or TileBuildSound>,
    // pub mined_sound: Option<Sound>,
    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub walking_speed_modifier: f64,
    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub vehicle_friction_modifier: f64,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub decorative_removal_probability: f32,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_neighbors: FactorioArray<TileID>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub needs_correction: bool,

    // pub minable: Option<MinableProperties>,
    pub next_direction: Option<TileID>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub can_be_part_of_blueprint: bool,

    pub effect: Option<String>,

    // pub trigger_effect: Option<TriggerEffect>,
    pub scorch_mark_color: Option<Color>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub check_collision_with_entities: bool,

    pub effect_color_secondary: Option<Color>,
    pub effect_is_opaque: Option<bool>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transitions: FactorioArray<TileTransitionsToTiles>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transitions_between_transitions: FactorioArray<TileTransitionsBetweenTransitions>,

    // pub autoplace: AutoplaceSpecification,
    pub placeable_by: Option<PlaceableBy>,
}

/// [`Types/TileRenderLayer`](https://lua-api.factorio.com/latest/types/TileRenderLayer.html)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TileRenderLayer {
    Zero,
    Water,
    WaterOverlay,
    Ground,
    Top,
}

/// [`Types/TileTransitions`](https://lua-api.factorio.com/latest/types/TileTransitions.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct TileTransitions<T> {
    #[serde(flatten)]
    child: T,
}

impl<T> std::ops::Deref for TileTransitions<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

/// [`Prototypes/TilePrototype/TileTransitionsVariants`](https://lua-api.factorio.com/latest/prototypes/TilePrototype.html#variants)
pub type TileTransitionsVariants = TileTransitions<TileTransitionsVariantsData>;

/// [`Prototypes/TilePrototype/TileTransitionsVariants`](https://lua-api.factorio.com/latest/prototypes/TilePrototype.html#variants)
#[derive(Debug, Deserialize, Serialize)]
pub struct TileTransitionsVariantsData {
    pub main: FactorioArray<TileSpriteWithProbability>,
    pub material_background: Option<TileSprite>,
}

/// [`Prototypes/TilePrototype/TileTransitionsToTiles`](https://lua-api.factorio.com/latest/prototypes/TilePrototype.html#transitions)
pub type TileTransitionsToTiles = TileTransitions<TileTransitionsToTilesData>;

/// [`Prototypes/TilePrototype/TileTransitionsToTiles`](https://lua-api.factorio.com/latest/prototypes/TilePrototype.html#transitions)
#[derive(Debug, Deserialize, Serialize)]
pub struct TileTransitionsToTilesData {
    pub to_tiles: FactorioArray<TileID>,
    pub transition_group: u8,
}

/// [`Prototypes/TilePrototype/TileTransitionsBetweenTransitions`](https://lua-api.factorio.com/latest/prototypes/TilePrototype.html#transitions_between_transitions)
pub type TileTransitionsBetweenTransitions = TileTransitions<TileTransitionsBetweenTransitionsData>;

/// [`Prototypes/TilePrototype/TileTransitionsBetweenTransitions`](https://lua-api.factorio.com/latest/prototypes/TilePrototype.html#transitions_between_transitions)
#[derive(Debug, Deserialize, Serialize)]
pub struct TileTransitionsBetweenTransitionsData {
    pub transition_group1: u8,
    pub transition_group2: u8,
}

namespace_struct! {
    AllTypes,
    TileID,
    "tile"
}
