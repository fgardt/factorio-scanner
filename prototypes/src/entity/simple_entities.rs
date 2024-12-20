use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{helper, EntityWithHealthPrototype, EntityWithOwnerPrototype};
use mod_util::UsedMods;
use types::*;

// TODO: implement rendering for simple entities

/// [`Prototypes/SimpleEntityPrototype`](https://lua-api.factorio.com/latest/prototypes/SimpleEntityPrototype.html)
pub type SimpleEntityPrototype = EntityWithHealthPrototype<SimpleEntityData>;

/// [`Prototypes/SimpleEntityPrototype`](https://lua-api.factorio.com/latest/prototypes/SimpleEntityPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleEntityData {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub count_as_rock_for_filtered_deconstruction: bool,

    pub render_layer: Option<RenderLayer>,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub secondary_draw_order: i8,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub random_animation_offset: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub random_variation_on_create: bool,

    #[serde(flatten)]
    pub graphics: Option<SimpleEntityGraphics>,

    pub lower_render_layer: Option<RenderLayer>,
    pub lower_pictures: Option<SpriteVariations>,
    // pub stateless_visualisation_variations: FactorioArray<StatelessVisualisations>,
}

impl super::Renderable for SimpleEntityData {
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SimpleEntityGraphics {
    Pictures { pictures: SpriteVariations },
    Picture { picture: Sprite4Way },
    Animations { animations: AnimationVariations },
}

/// [`Prototypes/SimpleEntityWithOwnerPrototype`](https://lua-api.factorio.com/latest/prototypes/SimpleEntityWithOwnerPrototype.html)
pub type SimpleEntityWithOwnerPrototype = EntityWithOwnerPrototype<SimpleEntityWithOwnerData>;

/// [`Prototypes/SimpleEntityWithOwnerPrototype`](https://lua-api.factorio.com/latest/prototypes/SimpleEntityWithOwnerPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleEntityWithOwnerData {
    // TODO: defaults
    pub render_layer: Option<RenderLayer>,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub secondary_draw_order: i8,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub random_animation_offset: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub random_variation_on_create: bool,

    #[serde(flatten)]
    pub graphics: Option<SimpleEntityGraphics>,

    pub lower_render_layer: Option<RenderLayer>,
    pub lower_pictures: Option<SpriteVariations>,

    #[serde(
        default = "ForceCondition::all",
        skip_serializing_if = "ForceCondition::is_all"
    )]
    pub force_visibility: ForceCondition,
    // pub stateless_visualisation_variations: FactorioArray<StatelessVisualisations>,
}

impl super::Renderable for SimpleEntityWithOwnerData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = match self.graphics.as_ref()? {
            SimpleEntityGraphics::Pictures { pictures } => pictures.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            ),
            SimpleEntityGraphics::Picture { picture } => picture.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            ),
            SimpleEntityGraphics::Animations { animations } => animations.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            ),
        }?;

        render_layers.add_entity(res, &options.position);
        Some(())
    }
}

/// [`Prototypes/SimpleEntityWithForcePrototype`](https://lua-api.factorio.com/latest/prototypes/SimpleEntityWithForcePrototype.html)
///
/// The only difference to `SimpleEntityWithOwnerPrototype` is that `is_military_target` defaults to `true` which is not relevant -> difference is not implemented.
pub type SimpleEntityWithForcePrototype = EntityWithOwnerPrototype<SimpleEntityWithForceData>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleEntityWithForceData(SimpleEntityWithOwnerData);

impl std::ops::Deref for SimpleEntityWithForceData {
    type Target = SimpleEntityWithOwnerData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl super::Renderable for SimpleEntityWithForceData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        self.0
            .render(options, used_mods, render_layers, image_cache)
    }
}
