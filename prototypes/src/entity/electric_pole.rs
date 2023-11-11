use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/ElectricPolePrototype`](https://lua-api.factorio.com/latest/prototypes/ElectricPolePrototype.html)
pub type ElectricPolePrototype = EntityWithOwnerPrototype<ElectricPoleData>;

/// [`Prototypes/ElectricPolePrototype`](https://lua-api.factorio.com/latest/prototypes/ElectricPolePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ElectricPoleData {
    pub pictures: RotatedSprite,
    pub supply_area_distance: f64,
    pub connection_points: FactorioArray<WireConnectionPoint>,

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
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> crate::RenderOutput {
        let res = self.pictures.render(
            render_layers.scale(),
            used_mods,
            image_cache,
            &options.into(),
        )?;

        render_layers.add_entity(res, &options.position);

        Some(())
    }
}
