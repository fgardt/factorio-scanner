use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{helper, EntityWithOwnerPrototype};
use crate::data_raw::types::*;

/// [`Prototypes/OffshorePumpPrototype`](https://lua-api.factorio.com/latest/prototypes/OffshorePumpPrototype.html)
pub type OffshorePumpPrototype = EntityWithOwnerPrototype<OffshorePumpData>;

/// [`Prototypes/OffshorePumpPrototype`](https://lua-api.factorio.com/latest/prototypes/OffshorePumpPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct OffshorePumpData {
    pub fluid_box: FluidBox,
    pub pumping_speed: f64,
    pub fluid: FluidID,

    #[serde(flatten)]
    pub graphics: OffshorePumpGraphicsVariant,

    #[serde(
        default = "helper::f64_quarter",
        skip_serializing_if = "helper::is_quarter_f64"
    )]
    pub min_perceived_performance: f64,

    pub fluid_box_tile_collision_test: Option<CollisionMask>,
    pub adjacent_tile_collision_test: Option<CollisionMask>,
    pub center_collision_mask: Option<CollisionMask>,
    pub adjacent_tile_collision_box: Option<BoundingBox>,
    pub placeable_position_visualization: Option<Sprite>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub remove_on_tile_collision: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub always_draw_fluid: bool,

    pub check_bounding_box_collides_with_tiles: Option<bool>,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub circuit_wire_max_distance: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_copper_wires: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_circuit_wires: bool,

    pub circuit_wire_connection_points: Option<(
        WireConnectionPoint,
        WireConnectionPoint,
        WireConnectionPoint,
        WireConnectionPoint,
    )>,
    pub circuit_connector_sprites: Option<(
        CircuitConnectorSprites,
        CircuitConnectorSprites,
        CircuitConnectorSprites,
        CircuitConnectorSprites,
    )>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OffshorePumpGraphicsVariant {
    Deprecated {
        picture: Sprite4Way,
    },
    GraphicsSet {
        graphics_set: OffshorePumpGraphicsSet,
    },
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct OffshorePumpGraphicsSet {
    pub animation: Animation4Way,

    // TODO: default value
    pub base_render_layer: Option<RenderLayer>,

    #[serde(default = "helper::i8_1", skip_serializing_if = "helper::is_1_i8")]
    pub underwater_layer_offset: i8,

    pub fluid_animation: Option<Animation4Way>,
    pub glass_pictures: Option<Sprite4Way>,
    pub base_pictures: Option<Sprite4Way>,
    pub underwater_pictures: Option<Sprite4Way>,
}
