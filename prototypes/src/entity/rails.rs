use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/RailPrototype`](https://lua-api.factorio.com/latest/prototypes/RailPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct RailPrototype<T: super::Renderable>(EntityWithOwnerPrototype<RailData<T>>);

impl<T: super::Renderable> super::Renderable for RailPrototype<T> {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Prototypes/RailPrototype`](https://lua-api.factorio.com/latest/prototypes/RailPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RailData<T: super::Renderable> {
    pub pictures: RailPictureSet,

    #[serde(flatten)]
    pub child: T,
    // not implemented
    // pub walking_sound: Option<Sound>,
}

impl<T: super::Renderable> super::Renderable for RailData<T> {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Prototypes/CurvedRailPrototype`](https://lua-api.factorio.com/latest/prototypes/CurvedRailPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct CurvedRailPrototype(RailPrototype<CurvedRailData>);

impl super::Renderable for CurvedRailPrototype {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        None
    }
}

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
#[derive(Debug, Deserialize, Serialize)]
pub struct StraightRailPrototype(RailPrototype<StraightRailData>);

impl super::Renderable for StraightRailPrototype {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        None
    }
}

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
