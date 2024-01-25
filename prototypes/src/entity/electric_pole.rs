use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{EntityWithOwnerPrototype, WireEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/ElectricPolePrototype`](https://lua-api.factorio.com/latest/prototypes/ElectricPolePrototype.html)
pub type ElectricPolePrototype = EntityWithOwnerPrototype<WireEntityData<ElectricPoleData>>;

/// [`Prototypes/ElectricPolePrototype`](https://lua-api.factorio.com/latest/prototypes/ElectricPolePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ElectricPoleData {
    pub pictures: RotatedSprite,
    pub supply_area_distance: f64,

    pub radius_visualisation_picture: Option<Sprite>,
    pub active_picture: Option<Sprite>,

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
    ) -> super::RenderOutput {
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
