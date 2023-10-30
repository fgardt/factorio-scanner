use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use types::*;

/// [`Prototypes/ElectricPolePrototype`](https://lua-api.factorio.com/latest/prototypes/ElectricPolePrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct ElectricPolePrototype(EntityWithOwnerPrototype<ElectricPoleData>);

impl super::Renderable for ElectricPolePrototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        self.0.render(options)
    }
}

/// [`Prototypes/ElectricPolePrototype`](https://lua-api.factorio.com/latest/prototypes/ElectricPolePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ElectricPoleData {
    pub pictures: RotatedSprite,
    pub supply_area_distance: f64,
    pub connection_points: Vec<WireConnectionPoint>,

    pub radius_visualisation_picture: Option<Sprite>,
    pub active_picture: Option<Sprite>,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub maximum_wire_distance: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_copper_wires: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_circuit_wires: bool,

    pub light: Option<LightDefinition>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub track_coverage_during_build_by_moving: bool,
}

impl super::Renderable for ElectricPoleData {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        self.pictures
            .render(options.factorio_dir, &options.used_mods, &options.into())
    }
}
