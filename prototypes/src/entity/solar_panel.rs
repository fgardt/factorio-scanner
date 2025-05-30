use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_with::skip_serializing_none;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/SolarPanelPrototype`](https://lua-api.factorio.com/latest/prototypes/SolarPanelPrototype.html)
pub type SolarPanelPrototype = EntityWithOwnerPrototype<SolarPanelData>;

/// [`Prototypes/SolarPanelPrototype`](https://lua-api.factorio.com/latest/prototypes/SolarPanelPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SolarPanelData {
    pub energy_source: ElectricEnergySource,
    pub picture: Option<SpriteVariations>,
    pub production: Energy,
    pub overlay: Option<SpriteVariations>,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub performance_at_day: f64,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub performance_at_night: f64,
    #[serde(
        default = "default_solar_coefficient_property",
        skip_serializing_if = "is_default_solar_coefficient_property"
    )]
    pub solar_coefficient_property: SurfacePropertyID,
}

fn default_solar_coefficient_property() -> SurfacePropertyID {
    SurfacePropertyID::new("solar-panel")
}

fn is_default_solar_coefficient_property(solar_coefficient_property: &SurfacePropertyID) -> bool {
    *solar_coefficient_property == default_solar_coefficient_property()
}

impl super::Renderable for SolarPanelData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self.picture.as_ref()?.render(
            render_layers.scale(),
            used_mods,
            image_cache,
            &options.into(),
        )?;

        render_layers.add_entity(res, &options.position);

        Some(())
    }
}
