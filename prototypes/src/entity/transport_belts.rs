use std::ops::Deref;

use image::{DynamicImage, GenericImageView};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;
use tracing::warn;

use super::{EntityWithOwnerPrototype, WireEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/TransportBeltConnectablePrototype`](https://lua-api.factorio.com/latest/prototypes/TransportBeltConnectablePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct TransportBeltConnectableData<G>
where
    G: super::RenderableGraphics,
{
    pub belt_animation_set: Option<Box<G>>,

    pub speed: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub animation_speed_coefficient: f64,
}

impl<G> super::Renderable for TransportBeltConnectableData<G>
where
    G: super::RenderableGraphics<RenderOpts: for<'a> From<&'a super::RenderOpts>>,
{
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self.belt_animation_set.as_ref().and_then(|bas| {
            bas.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        })?;

        render_layers.add_entity(split_belt(res, options), &options.position);

        Some(())
    }
}

fn split_belt(
    (img, shift): (DynamicImage, Vector),
    options: &super::RenderOpts,
) -> (DynamicImage, Vector) {
    let Some(underground_in) = options.underground_in else {
        return (img, shift);
    };

    // figure out which half to keep
    let (width, height) = img.dimensions();
    let (tx, ty, w, h, sx, sy) = {
        let dir = if underground_in {
            options.direction.flip()
        } else {
            options.direction
        };

        match dir {
            Direction::North => (0, 0, width, height.div_ceil(2), 0.0, -0.5),
            Direction::East => (width / 2, 0, width.div_ceil(2), height, 0.5, 0.0),
            Direction::South => (0, height / 2, width, height.div_ceil(2), 0.0, 0.5),
            Direction::West => (0, 0, width.div_ceil(2), height, -0.5, 0.0),
            _ => {
                warn!("belts only support cardinal directions");
                return (img, shift);
            }
        }
    };

    // let mut res = DynamicImage::new(w, h, img.color());
    // overlay(&mut res, &img.crop_imm(tx, ty, w, h), tx.into(), ty.into());

    (img.crop_imm(tx, ty, w, h), shift + Vector::new(sx, sy))
}

/// [`Prototypes/LinkedBeltPrototype`](https://lua-api.factorio.com/latest/prototypes/LinkedBeltPrototype.html)
pub type LinkedBeltPrototype = EntityWithOwnerPrototype<LinkedBeltData>;

/// [`Prototypes/LinkedBeltPrototype`](https://lua-api.factorio.com/latest/prototypes/LinkedBeltPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkedBeltData {
    pub structure: UndergroundBeltStructure,

    pub structure_render_layer: Option<RenderLayer>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_clone_connection: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_blueprint_connection: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub allow_side_loading: bool,

    #[serde(flatten)]
    parent: TransportBeltConnectableData<TransportBeltAnimationSet>,
}

impl Deref for LinkedBeltData {
    type Target = TransportBeltConnectableData<TransportBeltAnimationSet>;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl super::Renderable for LinkedBeltData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        self.parent
            .render(options, used_mods, render_layers, image_cache);

        self.structure
            .render(options, used_mods, render_layers, image_cache)
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        self.parent.fluid_box_connections(options)
    }

    fn heat_buffer_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        self.parent.heat_buffer_connections(options)
    }
}

/// [`Prototypes/LoaderPrototype`](https://lua-api.factorio.com/latest/prototypes/LoaderPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct LoaderData {
    pub structure: LoaderStructure,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub filter_count: u8,

    // TODO: default
    pub structure_render_layer: Option<RenderLayer>,
    pub circuit_connector_layer: Option<RenderLayer>,

    #[serde(
        default = "helper::f64_1_5",
        skip_serializing_if = "helper::is_1_5_f64"
    )]
    pub container_distance: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_rail_interaction: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_container_interaction: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub per_lane_filters: bool,

    #[serde(default = "helper::u8_1", skip_serializing_if = "helper::is_1_u8")]
    pub max_belt_stack_size: u8,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub adjustable_belt_stack_size: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub wait_for_full_stack: bool,

    //pub belt_length: f64, // -> moved to specific variants
    pub energy_source: Option<AnyEnergySource>, // any except burner
    pub energy_per_item: Option<Energy>,

    #[serde(flatten)]
    parent: TransportBeltConnectableData<TransportBeltAnimationSet>,
}

impl Deref for LoaderData {
    type Target = TransportBeltConnectableData<TransportBeltAnimationSet>;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl super::Renderable for LoaderData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        self.structure
            .render(options, used_mods, render_layers, image_cache)
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        self.parent.fluid_box_connections(options)
    }

    fn heat_buffer_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        self.parent.heat_buffer_connections(options)
    }
}

// used for loaders, linked belts and undergrounds
/// [`Types/LoaderStructure`](https://lua-api.factorio.com/latest/types/LoaderStructure.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct LoaderStructure {
    pub direction_in: Option<Sprite4Way>,
    pub direction_out: Option<Sprite4Way>,

    pub back_patch: Option<Sprite4Way>,
    pub front_patch: Option<Sprite4Way>,

    pub frozen_patch_in: Option<Sprite4Way>,
    pub frozen_patch_out: Option<Sprite4Way>,
}

impl super::Renderable for LoaderStructure {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = if options.underground_in.unwrap_or_default() {
            self.direction_in.as_ref().and_then(|d_in| {
                d_in.render(
                    render_layers.scale(),
                    used_mods,
                    image_cache,
                    &options.into(),
                )
            })
        } else {
            let flipped_opts = &super::RenderOpts {
                direction: options.direction.flip(),
                ..options.clone()
            };
            self.direction_out.as_ref().and_then(|d_out| {
                d_out.render(
                    render_layers.scale(),
                    used_mods,
                    image_cache,
                    &flipped_opts.into(),
                )
            })
        }?;

        render_layers.add_entity(res, &options.position);
        Some(())
    }
}

/// [`Prototypes/Loader1x1Prototype`](https://lua-api.factorio.com/latest/prototypes/Loader1x1Prototype.html)
pub type Loader1x1Prototype = EntityWithOwnerPrototype<WireEntityData<Loader1x1Data>>;

/// [`Prototypes/Loader1x1Prototype`](https://lua-api.factorio.com/latest/prototypes/Loader1x1Prototype.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct Loader1x1Data {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub belt_length: f64,

    #[serde(flatten)]
    parent: LoaderData,
}

impl Deref for Loader1x1Data {
    type Target = LoaderData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl super::Renderable for Loader1x1Data {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        // TODO: render short end piece instead of full belt
        self.parent
            .parent
            .render(options, used_mods, render_layers, image_cache);

        self.parent
            .render(options, used_mods, render_layers, image_cache)
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        self.parent.fluid_box_connections(options)
    }

    fn heat_buffer_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        self.parent.heat_buffer_connections(options)
    }
}

/// [`Prototypes/Loader1x2Prototype`](https://lua-api.factorio.com/latest/prototypes/Loader1x2Prototype.html)
pub type Loader1x2Prototype = EntityWithOwnerPrototype<WireEntityData<Loader1x2Data>>;

// this is required since the namespace_struct macro would not be able to map "loader" to "Loader1x2Prototype" otherwise
/// [`Prototypes/Loader1x2Prototype`](https://lua-api.factorio.com/latest/prototypes/Loader1x2Prototype.html)
pub type LoaderPrototype = Loader1x2Prototype;

/// [`Prototypes/Loader1x2Prototype`](https://lua-api.factorio.com/latest/prototypes/Loader1x2Prototype.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct Loader1x2Data {
    #[serde(default = "helper::f64_05", skip_serializing_if = "helper::is_05_f64")]
    pub belt_length: f64,

    #[serde(flatten)]
    parent: LoaderData,
}

impl Deref for Loader1x2Data {
    type Target = LoaderData;

    fn deref(&self) -> &Self::Target {
        unimplemented!()
    }
}

impl super::Renderable for Loader1x2Data {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let dir = if options.underground_in.unwrap_or_default() {
            options.direction.flip()
        } else {
            options.direction
        };
        let offset: MapPosition = (dir.get_offset() * 0.5).into();
        let pos_up = options.position + offset;

        self.parent.parent.render(
            &super::RenderOpts {
                position: pos_up,
                ..options.clone()
            },
            used_mods,
            render_layers,
            image_cache,
        );

        self.parent
            .render(options, used_mods, render_layers, image_cache)
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        self.parent.fluid_box_connections(options)
    }

    fn heat_buffer_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        self.parent.heat_buffer_connections(options)
    }
}

/// [`Prototypes/SplitterPrototype`](https://lua-api.factorio.com/latest/prototypes/SplitterPrototype.html)
pub type SplitterPrototype = EntityWithOwnerPrototype<SplitterData>;

/// [`Prototypes/SplitterPrototype`](https://lua-api.factorio.com/latest/prototypes/SplitterPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SplitterData {
    pub structure: Animation4Way,
    pub structure_patch: Option<Animation4Way>,
    pub frozen_patch: Option<Box<Sprite4Way>>,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub structure_animation_speed_coefficient: f64,

    #[serde(default = "helper::u32_10", skip_serializing_if = "helper::is_10_u32")]
    pub structure_animation_movement_cooldown: u32,

    pub related_transport_belt: Option<EntityID>,

    #[serde(flatten)]
    parent: TransportBeltConnectableData<TransportBeltAnimationSet>,
}

impl Deref for SplitterData {
    type Target = TransportBeltConnectableData<TransportBeltAnimationSet>;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl super::Renderable for SplitterData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let offset: MapPosition = (options.direction.right90().get_offset() * 0.5).into();
        let left_pos = options.position - offset;
        let right_pos = options.position + offset;

        self.parent.render(
            &super::RenderOpts {
                position: left_pos,
                ..options.clone()
            },
            used_mods,
            render_layers,
            image_cache,
        );

        self.parent.render(
            &super::RenderOpts {
                position: right_pos,
                ..options.clone()
            },
            used_mods,
            render_layers,
            image_cache,
        );

        let res = merge_renders(
            &[
                self.structure_patch.as_ref().and_then(|a| {
                    a.render(
                        render_layers.scale(),
                        used_mods,
                        image_cache,
                        &options.into(),
                    )
                }),
                self.structure.render(
                    render_layers.scale(),
                    used_mods,
                    image_cache,
                    &options.into(),
                ),
            ],
            render_layers.scale(),
        )?;

        render_layers.add_entity(res, &options.position);

        Some(())
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        self.parent.fluid_box_connections(options)
    }

    fn heat_buffer_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        self.parent.heat_buffer_connections(options)
    }
}

/// [`Prototypes/LaneSplitterPrototype`](https://lua-api.factorio.com/latest/prototypes/LaneSplitterPrototype.html)
pub type LaneSplitterPrototype = EntityWithOwnerPrototype<LaneSplitterData>;

/// [`Prototypes/LaneSplitterPrototype`](https://lua-api.factorio.com/latest/prototypes/LaneSplitterPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct LaneSplitterData {
    pub structure_animation_speed_coefficient: f64,
    pub structure_animation_movement_cooldown: u32,

    pub structure: Animation4Way,
    pub structure_patch: Option<Animation4Way>,

    #[serde(flatten)]
    parent: TransportBeltConnectableData<TransportBeltAnimationSet>,
}

impl Deref for LaneSplitterData {
    type Target = TransportBeltConnectableData<TransportBeltAnimationSet>;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl super::Renderable for LaneSplitterData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        self.parent
            .render(options, used_mods, render_layers, image_cache);

        let res = merge_renders(
            &[
                self.structure_patch.as_ref().and_then(|a| {
                    a.render(
                        render_layers.scale(),
                        used_mods,
                        image_cache,
                        &options.into(),
                    )
                }),
                self.structure.render(
                    render_layers.scale(),
                    used_mods,
                    image_cache,
                    &options.into(),
                ),
            ],
            render_layers.scale(),
        )?;

        render_layers.add_entity(res, &options.position);

        Some(())
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        self.parent.fluid_box_connections(options)
    }

    fn heat_buffer_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        self.parent.heat_buffer_connections(options)
    }
}

/// [`Prototypes/TransportBeltPrototype`](https://lua-api.factorio.com/latest/prototypes/TransportBeltPrototype.html)
pub type TransportBeltPrototype = EntityWithOwnerPrototype<WireEntityData<TransportBeltData>>;

/// [`Prototypes/TransportBeltPrototype`](https://lua-api.factorio.com/latest/prototypes/TransportBeltPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct TransportBeltData {
    pub connector_frame_sprites: TransportBeltConnectorFrame,
    pub related_underground_belt: Option<EntityID>,

    #[serde(flatten)]
    parent: TransportBeltConnectableData<TransportBeltAnimationSetWithCorners>,
}

impl Deref for TransportBeltData {
    type Target = TransportBeltConnectableData<TransportBeltAnimationSetWithCorners>;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl super::Renderable for TransportBeltData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self
            .parent
            .render(options, used_mods, render_layers, image_cache);

        // TODO: render special corner frames & edge walls
        if options.circuit_connected || options.logistic_connected {
            let res = self.connector_frame_sprites.frame_main.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )?;

            render_layers.add_entity(res, &options.position);
        }

        res
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        self.parent.fluid_box_connections(options)
    }

    fn heat_buffer_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        self.parent.heat_buffer_connections(options)
    }
}

/// [`Prototypes/UndergroundBeltPrototype`](https://lua-api.factorio.com/latest/prototypes/UndergroundBeltPrototype.html)
pub type UndergroundBeltPrototype = EntityWithOwnerPrototype<UndergroundBeltData>;

/// [`Prototypes/UndergroundBeltPrototype`](https://lua-api.factorio.com/latest/prototypes/UndergroundBeltPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct UndergroundBeltData {
    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub max_distance: u8,

    pub structure: Option<UndergroundBeltStructure>,
    pub underground_sprite: Option<Sprite>,
    pub underground_remove_belts_sprite: Option<Sprite>,
    pub max_distance_underground_remove_belts_sprite: Option<Sprite>,
    pub underground_collision_mask: Option<CollisionMaskConnector>,
    pub max_distance_tint: Option<Color>,

    #[serde(flatten)]
    parent: TransportBeltConnectableData<TransportBeltAnimationSet>,
}

impl Deref for UndergroundBeltData {
    type Target = TransportBeltConnectableData<TransportBeltAnimationSet>;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl super::Renderable for UndergroundBeltData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        self.parent
            .render(options, used_mods, render_layers, image_cache);

        self.structure
            .as_ref()
            .and_then(|s| s.render(options, used_mods, render_layers, image_cache))
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        self.parent.fluid_box_connections(options)
    }

    fn heat_buffer_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        self.parent.heat_buffer_connections(options)
    }
}

// used for undergrounds and linked belts
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct UndergroundBeltStructure {
    #[serde(flatten)]
    parent: LoaderStructure,

    pub direction_in_side_loading: Option<Sprite4Way>,
    pub direction_out_side_loading: Option<Sprite4Way>,
}

impl Deref for UndergroundBeltStructure {
    type Target = LoaderStructure;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl super::Renderable for UndergroundBeltStructure {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        self.parent
            .render(options, used_mods, render_layers, image_cache)
    }
}
