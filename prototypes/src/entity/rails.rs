use std::ops::Deref;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

pub trait RailDirectionPrototype {
    fn get_type(&self) -> RailDirectionType;
}

pub enum RailDirectionType {
    Straight,
    Curved,
}

/// [`Prototypes/RailPrototype`](https://lua-api.factorio.com/latest/prototypes/RailPrototype.html)
pub type RailPrototype<T> = EntityWithOwnerPrototype<RailData<T>>;

/// [`Prototypes/RailPrototype`](https://lua-api.factorio.com/latest/prototypes/RailPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RailData<T: RailDirectionPrototype> {
    pub pictures: RailPictureSet,

    #[serde(flatten)]
    child: T,
    // not implemented
    // pub walking_sound: Option<Sound>,
}

impl<T: RailDirectionPrototype> Deref for RailData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T: RailDirectionPrototype> super::Renderable for RailData<T> {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> crate::RenderOutput {
        match self.child.get_type() {
            RailDirectionType::Straight => {
                match options.direction {
                    Direction::North | Direction::South => self
                        .pictures
                        .straight_rail_vertical
                        .render(options, used_mods, render_layers, image_cache),
                    Direction::East | Direction::West => self
                        .pictures
                        .straight_rail_horizontal
                        .render(options, used_mods, render_layers, image_cache),
                    Direction::NorthWest => self.pictures.straight_rail_diagonal_left_top.render(
                        options,
                        used_mods,
                        render_layers,
                        image_cache,
                    ),
                    Direction::SouthEast => self
                        .pictures
                        .straight_rail_diagonal_right_bottom
                        .render(options, used_mods, render_layers, image_cache),
                    Direction::NorthEast => self.pictures.straight_rail_diagonal_right_top.render(
                        options,
                        used_mods,
                        render_layers,
                        image_cache,
                    ),
                    Direction::SouthWest => self
                        .pictures
                        .straight_rail_diagonal_left_bottom
                        .render(options, used_mods, render_layers, image_cache),
                }
            }
            RailDirectionType::Curved => {
                match options.direction {
                    Direction::North => self.pictures.curved_rail_vertical_left_bottom.render(
                        options,
                        used_mods,
                        render_layers,
                        image_cache,
                    ),
                    Direction::NorthEast => self.pictures.curved_rail_vertical_right_bottom.render(
                        options,
                        used_mods,
                        render_layers,
                        image_cache,
                    ),
                    Direction::East => self.pictures.curved_rail_horizontal_left_top.render(
                        options,
                        used_mods,
                        render_layers,
                        image_cache,
                    ),
                    Direction::SouthEast => self
                        .pictures
                        .curved_rail_horizontal_left_bottom
                        .render(options, used_mods, render_layers, image_cache),
                    Direction::South => self.pictures.curved_rail_vertical_right_top.render(
                        options,
                        used_mods,
                        render_layers,
                        image_cache,
                    ),
                    Direction::SouthWest => self.pictures.curved_rail_vertical_left_top.render(
                        options,
                        used_mods,
                        render_layers,
                        image_cache,
                    ),
                    Direction::West => self.pictures.curved_rail_horizontal_right_bottom.render(
                        options,
                        used_mods,
                        render_layers,
                        image_cache,
                    ),
                    Direction::NorthWest => self.pictures.curved_rail_horizontal_right_top.render(
                        options,
                        used_mods,
                        render_layers,
                        image_cache,
                    ),
                }
            }
        }
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

impl RailDirectionPrototype for CurvedRailData {
    fn get_type(&self) -> RailDirectionType {
        RailDirectionType::Curved
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

impl RailDirectionPrototype for StraightRailData {
    fn get_type(&self) -> RailDirectionType {
        RailDirectionType::Straight
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StraightBendType {
    Straight,
}

/// [`Types/RailPictureSet`](https://lua-api.factorio.com/latest/types/RailPictureSet.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct RailPictureSet {
    pub straight_rail_horizontal: RailPieceLayers,
    pub straight_rail_vertical: RailPieceLayers,
    pub straight_rail_diagonal_left_top: RailPieceLayers,
    pub straight_rail_diagonal_right_top: RailPieceLayers,
    pub straight_rail_diagonal_right_bottom: RailPieceLayers,
    pub straight_rail_diagonal_left_bottom: RailPieceLayers,
    pub curved_rail_vertical_left_top: RailPieceLayers,
    pub curved_rail_vertical_right_top: RailPieceLayers,
    pub curved_rail_vertical_right_bottom: RailPieceLayers,
    pub curved_rail_vertical_left_bottom: RailPieceLayers,
    pub curved_rail_horizontal_left_top: RailPieceLayers,
    pub curved_rail_horizontal_right_top: RailPieceLayers,
    pub curved_rail_horizontal_right_bottom: RailPieceLayers,
    pub curved_rail_horizontal_left_bottom: RailPieceLayers,
    pub rail_endings: Sprite8Way,
}

/// [`Types/RailPieceLayers`](https://lua-api.factorio.com/latest/types/RailPieceLayers.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RailPieceLayers {
    pub metals: SpriteVariations,
    pub backplates: SpriteVariations,
    pub ties: SpriteVariations,
    pub stone_path: SpriteVariations,

    pub stone_path_background: Option<SpriteVariations>,
    pub segment_visualisation_middle: Option<SpriteVariations>,
    pub segment_visualisation_ending_front: Option<SpriteVariations>,
    pub segment_visualisation_ending_back: Option<SpriteVariations>,
    pub segment_visualisation_continuing_front: Option<SpriteVariations>,
    pub segment_visualisation_continuing_back: Option<SpriteVariations>,
}

impl super::Renderable for RailPieceLayers {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> crate::RenderOutput {
        let mut empty = true;

        if let Some(path_background) = &self.stone_path_background {
            if let Some(res) = path_background.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            ) {
                empty = false;

                render_layers.add(
                    res,
                    &options.position,
                    crate::InternalRenderLayer::RailStonePathBackground,
                );
            }
        };

        if let Some(res) = self.stone_path.render(
            render_layers.scale(),
            used_mods,
            image_cache,
            &options.into(),
        ) {
            empty = false;

            render_layers.add(
                res,
                &options.position,
                crate::InternalRenderLayer::RailStonePath,
            );
        }

        if let Some(res) = self.ties.render(
            render_layers.scale(),
            used_mods,
            image_cache,
            &options.into(),
        ) {
            empty = false;

            render_layers.add(res, &options.position, crate::InternalRenderLayer::RailTies);
        }

        if let Some(res) = self.backplates.render(
            render_layers.scale(),
            used_mods,
            image_cache,
            &options.into(),
        ) {
            empty = false;

            render_layers.add(
                res,
                &options.position,
                crate::InternalRenderLayer::RailBackplate,
            );
        }

        if let Some(res) = self.metals.render(
            render_layers.scale(),
            used_mods,
            image_cache,
            &options.into(),
        ) {
            empty = false;

            render_layers.add(
                res,
                &options.position,
                crate::InternalRenderLayer::RailMetal,
            );
        }

        if empty {
            None
        } else {
            Some(())
        }
    }
}
