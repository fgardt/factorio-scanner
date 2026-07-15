use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    Direction, DirectionalRenderOpts, FactorioArray, LayeredSprite, RenderableGraphics, Sprite,
    SpriteNWaySheet,
};

#[cfg(feature = "graphics")]
use crate::merge_layers;

/// [`Types/Sprite4Way`](https://lua-api.factorio.com/latest/types/Sprite4Way.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[skip_serializing_none]
pub enum Sprite4Way {
    Sheets {
        sheets: FactorioArray<SpriteNWaySheet<4>>,
    },
    Sheet {
        sheet: SpriteNWaySheet<4>,
    },
    Struct {
        north: Box<Sprite>,
        east: Box<Sprite>,
        south: Box<Sprite>,
        west: Box<Sprite>,
    },
    Single(Sprite),
}

impl RenderableGraphics for Sprite4Way {
    type RenderOpts = DirectionalRenderOpts;

    #[cfg(feature = "graphics")]
    fn render(
        &self,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut crate::ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<super::GraphicsOutput> {
        use tracing::warn;

        match self {
            Self::Sheets { sheets } => merge_layers(sheets, scale, used_mods, image_cache, opts),
            Self::Sheet { sheet } => sheet.render(scale, used_mods, image_cache, opts),
            Self::Struct {
                north,
                east,
                south,
                west,
            } => match opts.direction {
                Direction::North => north.as_ref(),
                Direction::East => east.as_ref(),
                Direction::South => south.as_ref(),
                Direction::West => west.as_ref(),
                _ => {
                    warn!(
                        "Sprite4Way render called with invalid direction {:?}",
                        opts.direction
                    );
                    return None;
                }
            }
            .render(scale, used_mods, image_cache, opts),
            Self::Single(sprite) => sprite.render(scale, used_mods, image_cache, opts),
        }
    }
}

/// [`Types/Sprite8Way`](https://lua-api.factorio.com/latest/types/Sprite8Way.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[skip_serializing_none]
pub enum Sprite8Way {
    Sheets {
        sheets: FactorioArray<SpriteNWaySheet<8>>,
    },
    Sheet {
        sheet: SpriteNWaySheet<8>,
    },
    Struct {
        north: Box<Sprite>,
        north_east: Box<Sprite>,
        east: Box<Sprite>,
        south_east: Box<Sprite>,
        south: Box<Sprite>,
        south_west: Box<Sprite>,
        west: Box<Sprite>,
        north_west: Box<Sprite>,
    },
}

impl RenderableGraphics for Sprite8Way {
    type RenderOpts = DirectionalRenderOpts;

    #[cfg(feature = "graphics")]
    fn render(
        &self,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut crate::ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<super::GraphicsOutput> {
        use tracing::warn;

        match self {
            Self::Sheets { sheets } => merge_layers(sheets, scale, used_mods, image_cache, opts),
            Self::Sheet { sheet } => sheet.render(scale, used_mods, image_cache, opts),
            Self::Struct {
                north,
                north_east,
                east,
                south_east,
                south,
                south_west,
                west,
                north_west,
            } => match opts.direction {
                Direction::North => north.as_ref(),
                Direction::NorthEast => north_east.as_ref(),
                Direction::East => east.as_ref(),
                Direction::SouthEast => south_east.as_ref(),
                Direction::South => south.as_ref(),
                Direction::SouthWest => south_west.as_ref(),
                Direction::West => west.as_ref(),
                Direction::NorthWest => north_west.as_ref(),
                _ => {
                    warn!(
                        "Sprite8Way render called with invalid direction {:?}",
                        opts.direction
                    );
                    return None;
                }
            }
            .render(scale, used_mods, image_cache, opts),
        }
    }
}

/// [`Types/Sprite16Way`](https://lua-api.factorio.com/latest/types/Sprite16Way.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[skip_serializing_none]
pub enum Sprite16Way {
    Sheets {
        sheets: FactorioArray<SpriteNWaySheet<16>>,
    },
    Sheet {
        sheet: SpriteNWaySheet<16>,
    },
    Struct {
        north: Box<Sprite>,
        north_north_east: Box<Sprite>,
        north_east: Box<Sprite>,
        east_north_east: Box<Sprite>,
        east: Box<Sprite>,
        east_south_east: Box<Sprite>,
        south_east: Box<Sprite>,
        south_south_east: Box<Sprite>,
        south: Box<Sprite>,
        south_south_west: Box<Sprite>,
        south_west: Box<Sprite>,
        west_south_west: Box<Sprite>,
        west: Box<Sprite>,
        west_north_west: Box<Sprite>,
        north_west: Box<Sprite>,
        north_north_west: Box<Sprite>,
    },
}

impl RenderableGraphics for Sprite16Way {
    type RenderOpts = DirectionalRenderOpts;

    #[cfg(feature = "graphics")]
    fn render(
        &self,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut crate::ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<super::GraphicsOutput> {
        match self {
            Self::Sheets { sheets } => merge_layers(sheets, scale, used_mods, image_cache, opts),
            Self::Sheet { sheet } => sheet.render(scale, used_mods, image_cache, opts),
            Self::Struct {
                north,
                north_north_east,
                north_east,
                east_north_east,
                east,
                east_south_east,
                south_east,
                south_south_east,
                south,
                south_south_west,
                south_west,
                west_south_west,
                west,
                west_north_west,
                north_west,
                north_north_west,
            } => match opts.direction {
                Direction::North => north.as_ref(),
                Direction::NorthNorthEast => north_north_east.as_ref(),
                Direction::NorthEast => north_east.as_ref(),
                Direction::EastNorthEast => east_north_east.as_ref(),
                Direction::East => east.as_ref(),
                Direction::EastSouthEast => east_south_east.as_ref(),
                Direction::SouthEast => south_east.as_ref(),
                Direction::SouthSouthEast => south_south_east.as_ref(),
                Direction::South => south.as_ref(),
                Direction::SouthSouthWest => south_south_west.as_ref(),
                Direction::SouthWest => south_west.as_ref(),
                Direction::WestSouthWest => west_south_west.as_ref(),
                Direction::West => west.as_ref(),
                Direction::WestNorthWest => west_north_west.as_ref(),
                Direction::NorthWest => north_west.as_ref(),
                Direction::NorthNorthWest => north_north_west.as_ref(),
            }
            .render(scale, used_mods, image_cache, opts),
        }
    }
}

/// [`Types/LayeredSprite4Way`](https://lua-api.factorio.com/latest/types/LayeredSprite4Way.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LayeredSprite4Way {
    Struct {
        north: Box<LayeredSprite>,
        east: Box<LayeredSprite>,
        south: Box<LayeredSprite>,
        west: Box<LayeredSprite>,
    },
    Single(LayeredSprite),
}

impl RenderableGraphics for LayeredSprite4Way {
    type RenderOpts = DirectionalRenderOpts;

    #[cfg(feature = "graphics")]
    fn render(
        &self,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut crate::ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<super::GraphicsOutput> {
        use tracing::warn;

        match self {
            Self::Struct {
                north,
                east,
                south,
                west,
            } => match opts.direction {
                Direction::North => north.as_ref(),
                Direction::East => east.as_ref(),
                Direction::South => south.as_ref(),
                Direction::West => west.as_ref(),
                _ => {
                    warn!(
                        "LayeredSprite4Way render called with invalid direction {:?}",
                        opts.direction
                    );
                    return None;
                }
            }
            .render(scale, used_mods, image_cache, opts),
            Self::Single(sprite) => sprite.render(scale, used_mods, image_cache, opts),
        }
    }
}
