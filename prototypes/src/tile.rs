use std::collections::HashMap;

use mod_util::UsedMods;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;
use types::{
    AirbornePollutantID, Animation4Way, CollisionMaskConnector, Color, FactorioArray, FluidID,
    Icon, ImageCache, MapPosition, PlaceableBy, RenderableGraphics, TileEffectDefinitionID,
    TileGraphics, TileID, TileLightPictures, TileMainPictures, TileRenderOpts, TileSpriteLayout,
    Weight,
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

    pub build_animations: Option<Animation4Way>,
    pub build_animations_background: Option<Animation4Way>,
    pub built_animation_frame: Option<u32>,

    pub variants: TileTransitionsVariants,
    pub map_color: Color,

    #[serde(flatten)]
    pub icon: Option<Icon>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub lowland_fog: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub transition_overlay_layer_offset: u8,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub sprite_usage_surface: SpriteUsageSurfaceHint,

    pub layer_group: Option<TileRenderLayer>,
    pub transition_merges_with_tile: Option<TileID>,

    #[serde(default = "Color::white", skip_serializing_if = "Color::is_white")]
    pub effect_color: Color,
    #[serde(default = "Color::white", skip_serializing_if = "Color::is_white")]
    pub tint: Color,

    // pub particle_tints: Option<TileBasedParticleTints>,

    // pub walking_sound: Option<Sound>,
    // pub landing_steps_sound: Option<Sound>,
    // pub driving_sound: Option<Sound>,
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
    pub fluid: Option<FluidID>,

    pub next_direction: Option<TileID>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub can_be_part_of_blueprint: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub is_foundation: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub destroys_dropped_items: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allows_being_covered: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub searchable: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub max_health: f32,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub weight: Weight,

    // pub dying_explosion: ExplosionDefinition or array[ExplosionDefinition],
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub absorptions_per_second: HashMap<AirbornePollutantID, f64>,

    pub default_cover_tile: Option<TileID>,
    pub frozen_variant: Option<TileID>,
    pub thawed_variant: Option<TileID>,

    pub effect: Option<TileEffectDefinitionID>,

    // pub trigger_effect: Option<TriggerEffect>,
    // pub default_destroyed_dropped_item_trigger: Option<Trigger>,
    pub scorch_mark_color: Option<Color>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub check_collision_with_entities: bool,

    pub effect_color_secondary: Option<Color>,
    pub effect_is_opaque: Option<bool>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transitions: FactorioArray<TileTransitionsToTiles>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transitions_between_transitions: FactorioArray<TileTransitionsBetweenTransitions>,

    // pub autoplace: AutoplaceSpecification,
    pub placeable_by: Option<PlaceableBy>,
    // pub bound_decoratives: DecorativeID or array[DecorativeID],
    // pub ambient_sounds_group: Option<TileID>,
    // pub ambient_sounds: WorldAmbientSoundDefinition or array[WorldAmbientSoundDefinition],
}

/// [`Types/TileRenderLayer`](https://lua-api.factorio.com/latest/types/TileRenderLayer.html)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TileRenderLayer {
    Zero,
    Water,
    WaterOverlay,
    GroundNatural,
    GroundArtificial,
    Top,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SpriteUsageSurfaceHint {
    #[default]
    Any,
    Nauvis,
    Vulcanus,
    Gleba,
    Fulgora,
    Aquilo,
    Space,
}

// the only difference between `TileSpriteLayout` and `MaterialTextureParameters`
// is that 2 u32 fields are u8 instead
type MaterialTextureParameters = TileGraphics<TileSpriteLayout>;

/// [`TypesTileTransitionsVariants`](https://lua-api.factorio.com/latest/types/TileTransitionsVariants.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct TileTransitionsVariants {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub main: FactorioArray<TileMainPictures>,

    #[serde(default = "helper::u8_8", skip_serializing_if = "helper::is_8_u8")]
    pub material_texture_width_in_tiles: u8,
    #[serde(default = "helper::u8_8", skip_serializing_if = "helper::is_8_u8")]
    pub material_texture_height_in_tiles: u8,
    pub material_background: Option<MaterialTextureParameters>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub light: FactorioArray<TileLightPictures>,
    pub material_light: Option<MaterialTextureParameters>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub empty_transitions: bool,

    pub transition: Option<TileTransitions>,
}

/// [`Types/TileTransitions`](https://lua-api.factorio.com/latest/types/TileTransitions.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct TileTransitions {
    // lots of fields
}

/// [`Prototypes/TilePrototype/TileTransitionsToTiles`](https://lua-api.factorio.com/latest/prototypes/TilePrototype.html#transitions)
#[derive(Debug, Deserialize, Serialize)]
pub struct TileTransitionsToTiles {
    pub to_tiles: FactorioArray<TileID>,
    pub transition_group: u8,

    #[serde(flatten)]
    parent: TileTransitions,
}

impl std::ops::Deref for TileTransitionsToTiles {
    type Target = TileTransitions;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

/// [`Prototypes/TilePrototype/TileTransitionsBetweenTransitions`](https://lua-api.factorio.com/latest/prototypes/TilePrototype.html#transitions_between_transitions)
#[derive(Debug, Deserialize, Serialize)]
pub struct TileTransitionsBetweenTransitions {
    pub transition_group1: u8,
    pub transition_group2: u8,

    #[serde(flatten)]
    parent: TileTransitions,
}

impl std::ops::Deref for TileTransitionsBetweenTransitions {
    type Target = TileTransitions;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

namespace_struct! {
    AllTypes,
    TileID,
    "tile"
}
