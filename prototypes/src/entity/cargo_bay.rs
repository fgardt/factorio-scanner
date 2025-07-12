use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/CargoBayPrototype`](https://lua-api.factorio.com/latest/prototypes/CargoBayPrototype.html)
pub type CargoBayPrototype = EntityWithOwnerPrototype<CargoBayData>;

/// [`Prototypes/SpacePlatformHubPrototype`](https://lua-api.factorio.com/latest/prototypes/CargoBayPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct CargoBayData {
    pub graphics_set: Option<CargoBayConnectableGraphicsSet>,
    pub platform_graphics_set: Option<CargoBayConnectableGraphicsSet>,

    pub inventory_size_bonus: ItemStackIndex,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hatch_definitions: FactorioArray<CargoHatchDefinition>,
}

impl super::Renderable for CargoBayData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        // TODO: switch to platform set variant if a platform hub is present

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
