use std::ops::Rem;

use mod_util::UsedMods;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use crate::{
    FactorioArray, GraphicsOutput, ImageCache, LightDefinition, RealOrientation,
    RenderableGraphics, SimpleGraphicsRenderOpts, Sprite, Vector,
};

/// [`Types/WirePosition`](https://lua-api.factorio.com/latest/types/WirePosition.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WirePosition {
    pub copper: Option<Vector>,
    pub green: Option<Vector>,
    pub red: Option<Vector>,
}

/// [`Types/WireConnectionPoint`](https://lua-api.factorio.com/latest/types/WireConnectionPoint.html)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WireConnectionPoint {
    pub wire: WirePosition,
    pub shadow: WirePosition,
}

/// [`Types/CircuitConnectorSprites`](https://lua-api.factorio.com/latest/types/CircuitConnectorSprites.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitConnectorSprites {
    pub led_red: Sprite,
    pub led_green: Sprite,
    pub led_blue: Sprite,
    pub led_light: LightDefinition,

    pub connector_main: Option<Sprite>,
    pub connector_shadow: Option<Sprite>,

    pub wire_pins: Option<Sprite>,
    pub wire_pins_shadow: Option<Sprite>,

    pub led_blue_off: Option<Sprite>,
    pub led_blue_light_offset: Option<Vector>,
    pub red_green_led_light_offset: Option<Vector>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WireDrawFlags {
    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_copper_wires: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_circuit_wires: bool,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WireConnectionData {
    PowerPole {
        connection_points: FactorioArray<WireConnectionPoint>,

        #[serde(default, skip_serializing_if = "helper::is_default")]
        maximum_wire_distance: f64,

        #[serde(flatten)]
        draw_flags: WireDrawFlags,
    },
    PowerSwitch {
        left_wire_connection_point: Box<WireConnectionPoint>,
        right_wire_connection_point: Box<WireConnectionPoint>,
        circuit_wire_connection_point: Box<WireConnectionPoint>,

        #[serde(default, skip_serializing_if = "helper::is_default")]
        wire_max_distance: f64,

        #[serde(flatten)]
        draw_flags: WireDrawFlags,
    },
    Combinator {
        input_connection_points: Box<[WireConnectionPoint; 4]>,
        output_connection_points: Box<[WireConnectionPoint; 4]>,

        #[serde(default, skip_serializing_if = "helper::is_default")]
        circuit_wire_max_distance: f64,

        #[serde(flatten)]
        draw_flags: WireDrawFlags,
    },
    Oriented {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        circuit_wire_connection_points: FactorioArray<WireConnectionPoint>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        circuit_connector_sprites: FactorioArray<CircuitConnectorSprites>,

        #[serde(default, skip_serializing_if = "helper::is_default")]
        circuit_wire_max_distance: f64,

        #[serde(flatten)]
        draw_flags: WireDrawFlags,
    },
    Single {
        circuit_wire_connection_point: Option<Box<WireConnectionPoint>>,
        circuit_connector_sprites: Option<Box<CircuitConnectorSprites>>,

        #[serde(default, skip_serializing_if = "helper::is_default")]
        circuit_wire_max_distance: f64,

        #[serde(flatten)]
        draw_flags: WireDrawFlags,
    },
}

pub type GenericWireConnectionPoint = [Option<Box<WireConnectionPoint>>; 3];

impl WireConnectionData {
    #[must_use]
    pub const fn get_flags(&self) -> &WireDrawFlags {
        match self {
            Self::PowerPole { draw_flags, .. }
            | Self::PowerSwitch { draw_flags, .. }
            | Self::Combinator { draw_flags, .. }
            | Self::Single { draw_flags, .. }
            | Self::Oriented { draw_flags, .. } => draw_flags,
        }
    }

    #[must_use]
    pub const fn get_max_distance(&self) -> f64 {
        match self {
            Self::PowerPole {
                maximum_wire_distance,
                ..
            } => *maximum_wire_distance,
            Self::PowerSwitch {
                wire_max_distance, ..
            } => *wire_max_distance,
            Self::Combinator {
                circuit_wire_max_distance,
                ..
            }
            | Self::Single {
                circuit_wire_max_distance,
                ..
            }
            | Self::Oriented {
                circuit_wire_max_distance,
                ..
            } => *circuit_wire_max_distance,
        }
    }

    #[must_use]
    pub fn get_connection_point(
        &self,
        orientation: RealOrientation,
    ) -> Option<GenericWireConnectionPoint> {
        match self {
            Self::PowerSwitch {
                left_wire_connection_point: left_cp,
                right_wire_connection_point: right_cp,
                circuit_wire_connection_point: circuit_cp,
                ..
            } => {
                println!("storing WCPS: power-switch\n{left_cp:?}\n{right_cp:?}\n{circuit_cp:?}");
                Some([
                    Some(circuit_cp.clone()),
                    Some(left_cp.clone()),
                    Some(right_cp.clone()),
                ])
            }
            Self::Combinator {
                input_connection_points: in_cp,
                output_connection_points: out_cp,
                ..
            } => {
                let index = ((orientation + 0.125).rem(1.0) * 4.0).floor() as usize;
                Some([
                    Some(Box::new(in_cp[index])),
                    Some(Box::new(out_cp[index])),
                    None,
                ])
            }
            Self::Single {
                circuit_wire_connection_point: point,
                ..
            } => point.as_ref().map(|p| [Some(p.clone()), None, None]),
            Self::PowerPole {
                connection_points: points,
                ..
            }
            | Self::Oriented {
                circuit_wire_connection_points: points,
                ..
            } => {
                let directions = points.len();

                if directions == 0 {
                    None
                } else {
                    let directions = directions as f64;
                    let index =
                        ((orientation + (0.5 / directions)).rem(1.0) * directions).floor() as usize;

                    points.get(index).map(|p| [Some(Box::new(*p)), None, None])
                }
            }
        }
    }

    #[must_use]
    pub fn get_connector_sprites(
        &self,
        orientation: RealOrientation,
    ) -> Option<&CircuitConnectorSprites> {
        match self {
            Self::PowerPole { .. } | Self::PowerSwitch { .. } | Self::Combinator { .. } => None,
            Self::Single {
                circuit_connector_sprites,
                ..
            } => circuit_connector_sprites.as_deref(),
            Self::Oriented {
                circuit_connector_sprites,
                ..
            } => {
                let directions = circuit_connector_sprites.len();

                if directions == 0 {
                    None
                } else {
                    let directions = directions as f64;
                    let index =
                        ((orientation + (0.5 / directions)).rem(1.0) * directions).floor() as usize;

                    circuit_connector_sprites.get(index)
                }
            }
        }
    }

    #[must_use]
    pub fn render_connector(
        &self,
        orientation: RealOrientation,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.get_connector_sprites(orientation)
            .and_then(|s| s.connector_main.as_ref())
            .and_then(|s| {
                s.render(
                    scale,
                    used_mods,
                    image_cache,
                    &SimpleGraphicsRenderOpts::default(),
                )
            })
    }

    #[must_use]
    pub fn render_pins(
        &self,
        orientation: RealOrientation,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.get_connector_sprites(orientation)
            .and_then(|s| s.wire_pins.as_ref())
            .and_then(|s| {
                s.render(
                    scale,
                    used_mods,
                    image_cache,
                    &SimpleGraphicsRenderOpts::default(),
                )
            })
    }
}
