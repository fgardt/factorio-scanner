use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/MiningDrillPrototype`](https://lua-api.factorio.com/latest/prototypes/MiningDrillPrototype.html)
pub type MiningDrillPrototype = EntityWithOwnerPrototype<MiningDrillData>;

/// [`Prototypes/MiningDrillPrototype`](https://lua-api.factorio.com/latest/prototypes/MiningDrillPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct MiningDrillData {
    pub vector_to_place_result: Vector,
    pub resource_searching_radius: f64,
    pub mining_speed: f64,
    pub energy_usage: Energy,
    pub energy_source: AnyEnergySource,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_categories: FactorioArray<ResourceCategoryID>,

    pub output_fluid_box: Option<FluidBox>,
    pub input_fluid_box: Option<FluidBox>,

    pub animations: Option<Animation4Way>,
    pub graphics_set: Option<MiningDrillGraphicsSet>,
    pub wet_mining_graphics_set: Option<MiningDrillGraphicsSet>,
    pub base_picture: Option<Sprite4Way>,
    pub allowed_effects: Option<EffectTypeLimitation>,
    pub radius_visualisation_picture: Option<Sprite>,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub circuit_wire_max_distance: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_copper_wires: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_circuit_wires: bool,

    pub base_render_layer: Option<RenderLayer>,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub base_productivity: f64,

    pub monitor_visualization_tint: Option<Color>,

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

    pub module_specification: Option<ModuleSpecification>,
}

impl super::Renderable for MiningDrillData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> crate::RenderOutput {
        let res = if let Some(set) = self.graphics_set.as_ref() {
            set.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        } else {
            merge_renders(
                &[
                    self.base_picture.as_ref().and_then(|s| {
                        s.render(
                            render_layers.scale(),
                            used_mods,
                            image_cache,
                            &options.into(),
                        )
                    }),
                    self.animations.as_ref().and_then(|s| {
                        s.render(
                            render_layers.scale(),
                            used_mods,
                            image_cache,
                            &options.into(),
                        )
                    }),
                ],
                render_layers.scale(),
            )
        }?;

        render_layers.add_entity(res, &options.position);

        Some(())
    }
}
