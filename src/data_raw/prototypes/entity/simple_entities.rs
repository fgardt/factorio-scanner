use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{helper, EntityWithOwnerPrototype};
use crate::data_raw::types::*;

/// [`Prototypes/SimpleEntityWithOwnerPrototype`](https://lua-api.factorio.com/latest/prototypes/SimpleEntityWithOwnerPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleEntityWithOwnerPrototype(EntityWithOwnerPrototype<SimpleEntityData>);

impl super::Renderable for SimpleEntityWithOwnerPrototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Prototypes/SimpleEntityWithOwnerPrototype`](https://lua-api.factorio.com/latest/prototypes/SimpleEntityWithOwnerPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleEntityData {
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
    pub graphics: SimpleEntityGraphics,

    #[serde(
        default = "ForceCondition::all",
        skip_serializing_if = "ForceCondition::is_all"
    )]
    pub force_visibility: ForceCondition,
}

impl super::Renderable for SimpleEntityData {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SimpleEntityGraphics {
    Picture { picture: Sprite },
    Pictures { pictures: SpriteVariations },
    Animations { animations: AnimationVariations },
}

/// [`Prototypes/SimpleEntityWithForcePrototype`](https://lua-api.factorio.com/latest/prototypes/SimpleEntityWithForcePrototype.html)
///
/// The only difference to `SimpleEntityWithOwnerPrototype` is that `is_military_target` defaults to `true` which is not relevant -> difference is not implemented.
#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleEntityWithForcePrototype(SimpleEntityWithOwnerPrototype);

impl super::Renderable for SimpleEntityWithForcePrototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}
