use std::ops::Deref;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/RailPrototype`](https://lua-api.factorio.com/latest/prototypes/RailPrototype.html)
pub type RailPrototype<T> = EntityWithOwnerPrototype<RailData<T>>;

/// [`Prototypes/RailPrototype`](https://lua-api.factorio.com/latest/prototypes/RailPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RailData<T: super::Renderable> {
    pub pictures: RailPictureSet,

    #[serde(flatten)]
    child: T,
    // not implemented
    // pub walking_sound: Option<Sound>,
}

impl<T: super::Renderable> Deref for RailData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T: super::Renderable> super::Renderable for RailData<T> {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Prototypes/CurvedRailPrototype`](https://lua-api.factorio.com/latest/prototypes/CurvedRailPrototype.html)
pub type CurvedRailPrototype = RailPrototype<CurvedRailData>;

/// [`Prototypes/CurvedRailPrototype`](https://lua-api.factorio.com/latest/prototypes/CurvedRailPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct CurvedRailData {
    pub bending_type: Option<CurvedBendType>,
}

impl super::Renderable for CurvedRailData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        None
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CurvedBendType {
    Turn,
}

/// [`Prototypes/StraightRailPrototype`](https://lua-api.factorio.com/latest/prototypes/StraightRailPrototype.html)
pub type StraightRailPrototype = RailPrototype<StraightRailData>;

/// [`Prototypes/StraightRailPrototype`](https://lua-api.factorio.com/latest/prototypes/StraightRailPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct StraightRailData {
    pub bending_type: Option<StraightBendType>,
}

impl super::Renderable for StraightRailData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        None
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StraightBendType {
    Straight,
}
