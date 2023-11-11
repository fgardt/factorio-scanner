use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/TrainStopPrototype`](https://lua-api.factorio.com/latest/prototypes/TrainStopPrototype.html)
pub type TrainStopPrototype = EntityWithOwnerPrototype<TrainStopData>;

/// [`Prototypes/TrainStopPrototype`](https://lua-api.factorio.com/latest/prototypes/TrainStopPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct TrainStopData {
    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub animation_ticks_per_frame: u32,

    pub rail_overlay_animations: Option<Animation4Way>,
    pub animations: Option<Animation4Way>,
    pub top_animations: Option<Animation4Way>,

    pub default_train_stopped_signal: Option<SignalIDConnector>,
    pub default_trains_count_signal: Option<SignalIDConnector>,
    pub default_trains_limit_signal: Option<SignalIDConnector>,

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

    pub color: Option<Color>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub chart_name: bool,

    pub light1: Option<TrainStopLight>,
    pub light2: Option<TrainStopLight>,

    pub drawing_boxes: Option<TrainStopDrawingBoxes>,
    // TODO: overrides build_grid_size to 2
}

impl super::Renderable for TrainStopData {
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
pub struct TrainStopDrawingBoxes {
    pub north: BoundingBox,
    pub east: BoundingBox,
    pub south: BoundingBox,
    pub west: BoundingBox,
}
