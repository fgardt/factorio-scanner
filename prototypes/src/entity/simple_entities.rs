use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{helper, EntityWithHealthPrototype, EntityWithOwnerPrototype};
use mod_util::UsedMods;
use types::*;

// TODO: implement rendering for simple entities

/// [`Prototypes/SimpleEntityPrototype`](https://lua-api.factorio.com/latest/prototypes/SimpleEntityPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleEntityPrototype(EntityWithHealthPrototype<SimpleEntityData>);

impl Deref for SimpleEntityPrototype {
    type Target = EntityWithHealthPrototype<SimpleEntityData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SimpleEntityPrototype {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl super::Renderable for SimpleEntityPrototype {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Prototypes/SimpleEntityPrototype`](https://lua-api.factorio.com/latest/prototypes/SimpleEntityPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleEntityData {
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub count_as_rock_for_filtered_deconstruction: bool,

    pub render_layer: Option<RenderLayer>,

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_i8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub secondary_draw_order: i8,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub random_animation_offset: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub random_variation_on_create: bool,

    #[serde(flatten)]
    pub graphics: Option<SimpleEntityGraphics>,
}

impl super::Renderable for SimpleEntityData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        None
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SimpleEntityGraphics {
    Pictures { pictures: SpriteVariations },
    Picture { picture: Sprite },
    Animations { animations: AnimationVariations },
}

/// [`Prototypes/SimpleEntityWithOwnerPrototype`](https://lua-api.factorio.com/latest/prototypes/SimpleEntityWithOwnerPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleEntityWithOwnerPrototype(EntityWithOwnerPrototype<SimpleEntityWithOwnerData>);

impl Deref for SimpleEntityWithOwnerPrototype {
    type Target = EntityWithOwnerPrototype<SimpleEntityWithOwnerData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SimpleEntityWithOwnerPrototype {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl super::Renderable for SimpleEntityWithOwnerPrototype {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Prototypes/SimpleEntityWithOwnerPrototype`](https://lua-api.factorio.com/latest/prototypes/SimpleEntityWithOwnerPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleEntityWithOwnerData {
    // TODO: defaults
    pub render_layer: Option<RenderLayer>,

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_i8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub secondary_draw_order: i8,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub random_animation_offset: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub random_variation_on_create: bool,

    #[serde(flatten)]
    pub graphics: Option<SimpleEntityGraphics>,

    #[serde(
        default = "ForceCondition::all",
        skip_serializing_if = "ForceCondition::is_all"
    )]
    pub force_visibility: ForceCondition,
}

impl super::Renderable for SimpleEntityWithOwnerData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Prototypes/SimpleEntityWithForcePrototype`](https://lua-api.factorio.com/latest/prototypes/SimpleEntityWithForcePrototype.html)
///
/// The only difference to `SimpleEntityWithOwnerPrototype` is that `is_military_target` defaults to `true` which is not relevant -> difference is not implemented.
#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleEntityWithForcePrototype(SimpleEntityWithOwnerPrototype);

impl Deref for SimpleEntityWithForcePrototype {
    type Target = SimpleEntityWithOwnerPrototype;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SimpleEntityWithForcePrototype {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl super::Renderable for SimpleEntityWithForcePrototype {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        None
    }
}
