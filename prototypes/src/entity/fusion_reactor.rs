use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/FusionReactorPrototype`](https://lua-api.factorio.com/latest/prototypes/FusionReactorPrototype.html)
pub type FusionReactorPrototype = EntityWithOwnerPrototype<FusionReactorData>;

/// [`Prototypes/FusionReactorPrototype`](https://lua-api.factorio.com/latest/prototypes/FusionReactorPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct FusionReactorData {
    pub energy_source: ElectricEnergySource,
    pub burner: BurnerEnergySource,

    pub graphics_set: FusionReactorGraphicsSet,

    pub input_fluid_box: FluidBox,
    pub output_fluid_box: FluidBox,

    pub neighbour_connectable: Option<NeighbourConnectable>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub two_direction_only: bool,

    #[serde(default = "helper::f32_1", skip_serializing_if = "helper::is_1_f32")]
    pub neighbour_bonus: f32,

    pub power_input: Energy,
    pub max_fluid_usage: FluidAmount,

    pub target_temperature: Option<f32>,
    pub perceived_performance: Option<PerceivedPerformance>,
}

impl super::Renderable for FusionReactorData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let gs = &self.graphics_set;
        let res = gs.structure.as_ref().and_then(|s| {
            s.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        })?;

        render_layers.add_entity(res, &options.position);

        // TODO: render FB connections & neighbour connections

        Some(())
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<MapPosition> {
        let mut res = self
            .input_fluid_box
            .connection_points(options.direction, options.mirrored);

        res.extend(
            self.output_fluid_box
                .connection_points(options.direction, options.mirrored),
        );

        res
    }

    fn render_debug(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
    ) {
        self.input_fluid_box
            .render_debug(options, used_mods, render_layers);
        self.output_fluid_box
            .render_debug(options, used_mods, render_layers);
    }
}

/// [`Types/FusionReactorGraphicsSet`](https://lua-api.factorio.com/latest/types/FusionReactorGraphicsSet.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct FusionReactorGraphicsSet {
    pub structure: Option<Sprite4Way>,
    #[serde(
        default = "RenderLayer::object",
        skip_serializing_if = "RenderLayer::is_object"
    )]
    pub render_layer: RenderLayer,
    #[serde(default = "Color::white", skip_serializing_if = "Color::is_white")]
    pub default_fuel_glow_color: Color,
    pub light: Option<LightDefinition>,
    pub working_light_pictures: Option<Sprite4Way>,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub use_fuel_glow_color: bool,
    pub fusion_effect_uv_map: Option<Sprite>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub connections_graphics: FactorioArray<FusionReactorConnectionGraphics>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub direction_to_connection_graphics: HashMap<DirectionString, FactorioArray<u8>>,
    pub plasma_category: NeighbourConnectableConnectionCategory,
    pub water_reflection: Option<WaterReflectionDefinition>,
}

/// [`Types/FusionReactorConnectionGraphics`](https://lua-api.factorio.com/latest/types/FusionReactorConnectionGraphics.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct FusionReactorConnectionGraphics {
    pub pictures: Option<Animation>,
    pub working_light_pictures: Option<Animation>,
    pub fusion_effect_uv_map: Option<Sprite>,
}

/// [`Types/NeighbourConnectable`](https://lua-api.factorio.com/latest/types/NeighbourConnectable.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct NeighbourConnectable {
    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub affected_by_direction: bool,
    #[serde(default = "helper::f32_07", skip_serializing_if = "helper::is_07_f32")]
    pub neighbour_search_distance: f32,
    pub connections: FactorioArray<NeighbourConnectableConnectionDefinition>,
}

/// [`Types/NeighbourConnectableConnectionCategory`](https://lua-api.factorio.com/latest/types/NeighbourConnectableConnectionCategory.html)
pub type NeighbourConnectableConnectionCategory = String;

/// [`Types/NeighbourConnectableConnectionDefinition`](https://lua-api.factorio.com/latest/types/NeighbourConnectableConnectionDefinition.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct NeighbourConnectableConnectionDefinition {
    pub location: MapLocation,
    pub category: NeighbourConnectableConnectionCategory,
    pub neighbour_category: FactorioArray<NeighbourConnectableConnectionCategory>,
}

/// [`Types/NeighbourConnectableConnectionDefinition/MapLocation`](https://lua-api.factorio.com/latest/types/NeighbourConnectableConnectionDefinition.html#location)
#[derive(Debug, Deserialize, Serialize)]
pub struct MapLocation {
    pub position: MapPosition,
    pub direction: Direction,
}
