use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::{EntityWithOwnerPrototype, WireEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/SpacePlatformHubPrototype`](https://lua-api.factorio.com/latest/prototypes/SpacePlatformHubPrototype.html)
pub type SpacePlatformHubPrototype = EntityWithOwnerPrototype<WireEntityData<SpacePlatformHubData>>;

/// [`Prototypes/SpacePlatformHubPrototype`](https://lua-api.factorio.com/latest/prototypes/SpacePlatformHubPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct SpacePlatformHubData {
    pub graphics_set: Option<CargoBayConnectableGraphicsSet>,
    pub inventory_size: ItemStackIndex,
    pub dump_container: EntityID,

    pub default_speed_signal: Option<SignalIDConnector>,
    pub default_damage_taken_signal: Option<SignalIDConnector>,

    #[serde(default = "helper::f32_1", skip_serializing_if = "helper::is_1_f32")]
    pub platform_repair_speed_modifier: f32,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub weight: Weight,

    pub cargo_station_parameters: CargoStationParameters,
    // not implemented
    // pub surface_render_parameters: Option<SurfaceRenderParameters>,
    // pub persistent_ambient_sounds: Option<PersistentWorldAmbientSoundsDefinition>,
}

impl super::Renderable for SpacePlatformHubData {
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

        Some(())
    }
}
