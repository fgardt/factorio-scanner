use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use crate::{
    CollisionMaskConnector, Color, Direction, FactorioArray, FluidAmount,
    FluidBoxLinkedConnectionID, FluidID, MapPosition, RenderLayer, Sprite4Way,
};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum PipeConnectionFlowDirection {
    #[default]
    InputOutput,
    Input,
    Output,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum PipeConnectionType {
    #[default]
    Normal,
    Underground,
    Linked,
}

/// [`Types/PipeConnectionDefinition`](https://lua-api.factorio.com/latest/types/PipeConnectionDefinition.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct PipeConnectionDefinition {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub flow_direction: PipeConnectionFlowDirection,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub connection_type: PipeConnectionType,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enable_working_visualisation: FactorioArray<String>,

    pub direction: Option<Direction>,
    pub position: Option<MapPosition>,
    pub positions: Option<[MapPosition; 4]>,
    // pub connection_category
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub max_underground_distance: u8,
    pub max_distance_tint: Option<Color>,

    pub underground_collision_mask: Option<CollisionMaskConnector>,

    pub linked_connection_id: Option<FluidBoxLinkedConnectionID>,
}

/// [`Types/FluidBox/ProductionType`](https://lua-api.factorio.com/latest/types/FluidBox.html#production_type)
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum FluidBoxProductionType {
    #[default]
    None,
    // #[serde(rename = "None")]
    // None2,
    Input,
    InputOutput,
    Output,
}

/// [`Types/FluidBox.secondary_draw_orders`](https://lua-api.factorio.com/latest/types/FluidBox.html#secondary_draw_orders)
#[derive(Debug, Serialize, Deserialize)]
pub struct FluidBoxSecondaryDrawOrders {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "helper::truncating_opt_deserializer"
    )]
    pub north: Option<i8>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "helper::truncating_opt_deserializer"
    )]
    pub east: Option<i8>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "helper::truncating_opt_deserializer"
    )]
    pub south: Option<i8>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "helper::truncating_opt_deserializer"
    )]
    pub west: Option<i8>,
}

/// [`Types/FluidBox`](https://lua-api.factorio.com/latest/types/FluidBox.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct FluidBox {
    pub volume: FluidAmount,
    pub pipe_connections: FactorioArray<PipeConnectionDefinition>,
    pub filter: Option<FluidID>,
    pub render_layer: Option<RenderLayer>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub draw_only_when_connected: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub hide_connection_info: bool,

    pub pipe_covers: Option<Sprite4Way>,
    pub pipe_covers_frozen: Option<Sprite4Way>,
    pub pipe_picture: Option<Sprite4Way>,
    pub pipe_picture_frozen: Option<Sprite4Way>,
    pub mirrored_pipe_picture: Option<Sprite4Way>,
    pub mirrored_pipe_picture_frozen: Option<Sprite4Way>,

    pub minimum_temperature: Option<f64>,
    pub maximum_temperature: Option<f64>,

    pub max_pipeline_extent: Option<u32>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub production_type: FluidBoxProductionType,

    #[serde(default = "helper::i8_1", skip_serializing_if = "helper::is_1_i8")]
    pub secondary_draw_order: i8,
    pub secondary_draw_orders: Option<FluidBoxSecondaryDrawOrders>,

    pub always_draw_covers: Option<bool>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enable_working_visualisations: FactorioArray<String>,
}

impl FluidBox {
    #[must_use]
    pub fn connection_points(&self, direction: Direction, mirrored: bool) -> Vec<MapPosition> {
        self.pipe_connections
            .iter()
            .filter_map(|c| {
                if c.connection_type == PipeConnectionType::Linked {
                    return None;
                }

                let pos = if let Some(position) = c.position {
                    direction.rotate_vector(position.into())
                } else if let Some(positions) = c.positions {
                    let idx = direction.as_4way_idx()?;
                    positions[idx].into()
                } else {
                    return None;
                };

                let pos = if mirrored {
                    direction.mirror_vector(pos)
                } else {
                    pos
                };

                Some(pos.into())
            })
            .collect()
    }
}
