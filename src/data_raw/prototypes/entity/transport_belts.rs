use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{helper, EntityWithOwnerPrototype};
use crate::data_raw::types::*;

/// [`Prototypes/TransportBeltConnectablePrototype`](https://lua-api.factorio.com/latest/prototypes/TransportBeltConnectablePrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct TransportBeltConnectablePrototype<G: super::Renderable, T: super::Renderable>(
    EntityWithOwnerPrototype<TransportBeltConnectableData<G, T>>,
);

impl<G, T> super::Renderable for TransportBeltConnectablePrototype<G, T>
where
    G: super::Renderable,
    T: super::Renderable,
{
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        self.0.render(options)
    }
}

/// [`Prototypes/TransportBeltConnectablePrototype`](https://lua-api.factorio.com/latest/prototypes/TransportBeltConnectablePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct TransportBeltConnectableData<G, T> {
    pub speed: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub animation_speed_coefficient: f64,

    #[serde(flatten)]
    pub graphics_set: G,

    #[serde(flatten)]
    pub child: T,
}

impl<G, T> super::Renderable for TransportBeltConnectableData<G, T>
where
    G: super::Renderable,
    T: super::Renderable,
{
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        merge_renders(&[
            self.graphics_set.render(options),
            self.child.render(options),
        ])
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BeltGraphics {
    BeltAnimationSet {
        belt_animation_set: TransportBeltAnimationSet,
    },
    Individual {
        belt_horizontal: Animation,
        belt_vertical: Animation,

        ending_top: Animation,
        ending_bottom: Animation,
        ending_side: Animation,

        starting_top: Animation,
        starting_bottom: Animation,
        starting_side: Animation,

        ending_patch: Sprite4Way,

        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        ends_with_stopper: bool,
    },
}

impl super::Renderable for BeltGraphics {
    fn render(&self, options: &super::RenderOpts) -> Option<(image::DynamicImage, f64, Vector)> {
        match self {
            Self::BeltAnimationSet { belt_animation_set } => {
                belt_animation_set.render(options.factorio_dir, &options.used_mods, &options.into())
            }
            Self::Individual { .. } => None,
        }
    }
}

/// [`Prototypes/LinkedBeltPrototype`](https://lua-api.factorio.com/latest/prototypes/LinkedBeltPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct LinkedBeltPrototype(TransportBeltConnectablePrototype<BeltGraphics, LinkedBeltData>);

impl super::Renderable for LinkedBeltPrototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        self.0.render(options)
    }
}

/// [`Prototypes/LinkedBeltPrototype`](https://lua-api.factorio.com/latest/prototypes/LinkedBeltPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkedBeltData {
    pub structure: LinkedBeltStructure,

    pub structure_render_layer: Option<RenderLayer>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_clone_connection: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_blueprint_connection: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub allow_side_loading: bool,
    // TODO: collision_mask overridden
}

impl super::Renderable for LinkedBeltData {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        if options.underground_in.unwrap_or_default() {
            self.structure.direction_in.render(
                options.factorio_dir,
                &options.used_mods,
                &options.into(),
            )
        } else {
            let flipped_opts = &super::RenderOpts {
                direction: Some(options.direction.unwrap_or_default().flip()),
                ..options.clone()
            };
            self.structure.direction_out.render(
                options.factorio_dir,
                &options.used_mods,
                &flipped_opts.into(),
            )
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkedBeltStructure {
    pub direction_in: Sprite4Way,
    pub direction_out: Sprite4Way,

    pub back_patch: Option<Sprite4Way>,
    pub front_patch: Option<Sprite4Way>,

    pub direction_in_side_loading: Option<Sprite4Way>,
    pub direction_out_side_loading: Option<Sprite4Way>,
}

/// [`Prototypes/LoaderPrototype`](https://lua-api.factorio.com/latest/prototypes/LoaderPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct LoaderPrototype<T: super::Renderable>(
    TransportBeltConnectablePrototype<BeltGraphics, LoaderData<T>>,
);

impl<T: super::Renderable> super::Renderable for LoaderPrototype<T> {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        self.0.render(options)
    }
}

/// [`Prototypes/LoaderPrototype`](https://lua-api.factorio.com/latest/prototypes/LoaderPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct LoaderData<T: super::Renderable> {
    pub structure: LoaderStructure,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub filter_count: u8,

    // TODO: default
    pub structure_render_layer: Option<RenderLayer>,

    #[serde(
        default = "helper::f64_1_5",
        skip_serializing_if = "helper::is_1_5_f64"
    )]
    pub container_distance: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_rail_interaction: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_container_interaction: bool,

    //pub belt_length: f64, // -> moved to specific variants
    pub energy_source: Option<AnyEnergySource>, // any except burner
    pub energy_per_item: Option<Energy>,

    #[serde(flatten)]
    pub child: T,
}

impl<T: super::Renderable> super::Renderable for LoaderData<T> {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Types/LoaderStructure`](https://lua-api.factorio.com/latest/types/LoaderStructure.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct LoaderStructure {
    pub direction_in: Sprite4Way,
    pub direction_out: Sprite4Way,

    pub back_patch: Option<Sprite4Way>,
    pub front_patch: Option<Sprite4Way>,
}

/// [`Prototypes/Loader1x1Prototype`](https://lua-api.factorio.com/latest/prototypes/Loader1x1Prototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct Loader1x1Prototype(LoaderPrototype<Loader1x1Data>);

impl super::Renderable for Loader1x1Prototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        self.0.render(options)
    }
}

// TODO: loaders `belt_length` is not actually hardcoded but defaults to a internal hardcoded value instead..

/// [`Prototypes/Loader1x1Prototype`](https://lua-api.factorio.com/latest/prototypes/Loader1x1Prototype.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct Loader1x1Data {
    // hardcoded to 0, validate this?
    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub belt_length: f64,
}

impl super::Renderable for Loader1x1Data {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Prototypes/Loader1x1Prototype`](https://lua-api.factorio.com/latest/prototypes/Loader1x1Prototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct Loader1x2Prototype(LoaderPrototype<Loader1x2Data>);

impl super::Renderable for Loader1x2Prototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        self.0.render(options)
    }
}

/// [`Prototypes/Loader1x1Prototype`](https://lua-api.factorio.com/latest/prototypes/Loader1x1Prototype.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct Loader1x2Data {
    // hardcoded to 0.5, validate this?
    #[serde(
        default = "helper::f64_half",
        skip_serializing_if = "helper::is_half_f64"
    )]
    pub belt_length: f64,
}

impl super::Renderable for Loader1x2Data {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Prototypes/SplitterPrototype`](https://lua-api.factorio.com/latest/prototypes/SplitterPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct SplitterPrototype(TransportBeltConnectablePrototype<BeltGraphics, SplitterData>);

impl super::Renderable for SplitterPrototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        self.0.render(options)
    }
}

/// [`Prototypes/SplitterPrototype`](https://lua-api.factorio.com/latest/prototypes/SplitterPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SplitterData {
    pub structure: Animation4Way,
    pub structure_patch: Option<Animation4Way>,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub structure_animation_speed_coefficient: f64,

    #[serde(default = "helper::u32_10", skip_serializing_if = "helper::is_10_u32")]
    pub structure_animation_movement_cooldown: u32,
}

impl super::Renderable for SplitterData {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        // TODO: figure out how to render the 2 belts below the splitter

        merge_renders(&[
            self.structure_patch
                .as_ref()
                .and_then(|a| a.render(options.factorio_dir, &options.used_mods, &options.into())),
            self.structure
                .render(options.factorio_dir, &options.used_mods, &options.into()),
        ])
    }
}

/// [`Prototypes/TransportBeltPrototype`](https://lua-api.factorio.com/latest/prototypes/TransportBeltPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct TransportBeltPrototype(
    TransportBeltConnectablePrototype<BeltGraphicsWithCorners, TransportBeltData>,
);

impl super::Renderable for TransportBeltPrototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        self.0.render(options)
    }
}

/// [`Prototypes/TransportBeltPrototype`](https://lua-api.factorio.com/latest/prototypes/TransportBeltPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct TransportBeltData {
    pub connector_frame_sprites: TransportBeltConnectorFrame,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub circuit_wire_max_distance: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_copper_wires: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_circuit_wires: bool,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub circuit_wire_connection_points: Vec<WireConnectionPoint>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub circuit_connector_sprites: Vec<CircuitConnectorSprites>,

    pub related_underground_belt: Option<EntityID>,
}

impl super::Renderable for TransportBeltData {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BeltGraphicsWithCorners {
    BeltAnimationSetWithCorners {
        belt_animation_set: TransportBeltAnimationSetWithCorners,
    },
    Animations {
        animations: RotatedAnimation, // must have 12 animations
    },
}

impl super::Renderable for BeltGraphicsWithCorners {
    fn render(&self, options: &super::RenderOpts) -> Option<(image::DynamicImage, f64, Vector)> {
        match self {
            Self::BeltAnimationSetWithCorners { belt_animation_set } => {
                belt_animation_set.render(options.factorio_dir, &options.used_mods, &options.into())
            }
            Self::Animations { .. } => None,
        }
    }
}

/// [`Prototypes/UndergroundBeltPrototype`](https://lua-api.factorio.com/latest/prototypes/UndergroundBeltPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct UndergroundBeltPrototype(
    TransportBeltConnectablePrototype<BeltGraphics, UndergroundBeltData>,
);

impl super::Renderable for UndergroundBeltPrototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        self.0.render(options)
    }
}

/// [`Prototypes/UndergroundBeltPrototype`](https://lua-api.factorio.com/latest/prototypes/UndergroundBeltPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct UndergroundBeltData {
    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub max_distance: u8,

    pub structure: UndergroundBeltStructure,
    pub underground_sprite: Sprite,
    pub underground_remove_belts_sprite: Option<Sprite>,
}

impl super::Renderable for UndergroundBeltData {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        if options.underground_in.unwrap_or_default() {
            self.structure.direction_in.render(
                options.factorio_dir,
                &options.used_mods,
                &options.into(),
            )
        } else {
            let flipped_opts = &super::RenderOpts {
                direction: Some(options.direction.unwrap_or_default().flip()),
                ..options.clone()
            };
            self.structure.direction_out.render(
                options.factorio_dir,
                &options.used_mods,
                &flipped_opts.into(),
            )
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct UndergroundBeltStructure {
    pub direction_in: Sprite4Way,
    pub direction_out: Sprite4Way,
    pub back_patch: Option<Sprite4Way>,
    pub front_patch: Option<Sprite4Way>,
    pub direction_in_side_loading: Option<Sprite4Way>,
    pub direction_out_side_loading: Option<Sprite4Way>,
}
