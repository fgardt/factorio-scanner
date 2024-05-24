use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::{EnergyEntityData, EntityWithOwnerPrototype, FluidBoxEntityData, WireEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/PumpPrototype`](https://lua-api.factorio.com/latest/prototypes/PumpPrototype.html)
pub type PumpPrototype =
    EntityWithOwnerPrototype<WireEntityData<FluidBoxEntityData<EnergyEntityData<PumpData>>>>;

/// [`Prototypes/PumpPrototype`](https://lua-api.factorio.com/latest/prototypes/PumpPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct PumpData {
    pub energy_usage: Energy,
    pub pumping_speed: f64,
    pub animations: Animation4Way,

    #[serde(
        default = "helper::f64_1_64",
        skip_serializing_if = "helper::is_1_64_f64"
    )]
    pub fluid_wagon_connector_speed: f64,

    #[serde(
        default = "helper::f64_2_32",
        skip_serializing_if = "helper::is_2_32_f64"
    )]
    pub fluid_wagon_connector_alignment_tolerance: f64,

    #[serde(
        default = "helper::u8_1",
        skip_serializing_if = "helper::is_1_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub fluid_wagon_connector_frame_count: u8,

    pub fluid_animation: Option<Animation4Way>,
    pub glass_pictures: Option<Sprite4Way>,
    pub fluid_wagon_connector_graphics: Option<FluidWagonConnectorGraphics>,
}

impl super::Renderable for PumpData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self.animations.render(
            render_layers.scale(),
            used_mods,
            image_cache,
            &options.into(),
        )?;

        render_layers.add_entity(res, &options.position);

        Some(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FluidWagonConnectorGraphics {
    pub load_animations: PumpConnectorGraphics,
    pub unload_animations: PumpConnectorGraphics,
}
