use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::{EntityWithOwnerPrototype, WireEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/CargoLandingPadPrototype`](https://lua-api.factorio.com/latest/prototypes/CargoLandingPadPrototype.html)
pub type CargoLandingPadPrototype = EntityWithOwnerPrototype<WireEntityData<CargoLandingPadData>>;

/// [`Prototypes/SpacePlatformHubPrototype`](https://lua-api.factorio.com/latest/prototypes/CargoLandingPadPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct CargoLandingPadData {
    pub graphics_set: Option<CargoBayConnectableGraphicsSet>,
    pub inventory_size: ItemStackIndex,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub trash_inventory_size: ItemStackIndex,

    pub cargo_station_parameters: CargoStationParameters,

    pub robot_door: Option<RobotDoorSpecification>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub radar_range: u32,
    pub radar_visualisation_color: Option<Color>,
    pub radius_visualisation_picture: Option<Sprite>,
}

impl super::Entity for CargoLandingPadData {}

impl super::Renderable for CargoLandingPadData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let gs = self.graphics_set.as_ref()?;
        if let Some(anim) = gs.animation.as_ref().and_then(|a| {
            a.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        }) {
            render_layers.add_entity(anim, &options.position);
        }

        if let Some(pic) = gs.picture.as_ref().and_then(|p| {
            p.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        }) {
            render_layers.add_entity(pic, &options.position);
        }

        if let Some(door) = self
            .robot_door
            .as_ref()
            .and_then(|rd| rd.animation.as_ref())
            .and_then(|a| {
                a.render(
                    render_layers.scale(),
                    used_mods,
                    image_cache,
                    &options.into(),
                )
            })
        {
            render_layers.add_entity(door, &options.position);
        }

        Some(())
    }
}
