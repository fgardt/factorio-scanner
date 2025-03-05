use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::{EntityWithOwnerPrototype, WireEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/ProxyContainerPrototype`](https://lua-api.factorio.com/latest/prototypes/ProxyContainerPrototype.html)
pub type ProxyContainerPrototype = EntityWithOwnerPrototype<WireEntityData<ProxyContainerData>>;

/// [`Prototypes/ProxyContainerPrototype`](https://lua-api.factorio.com/latest/prototypes/ProxyContainerPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ProxyContainerData {
    pub picture: Option<Sprite>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_inventory_content: bool,
}

impl super::Renderable for ProxyContainerData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self.picture.as_ref().and_then(|p| {
            p.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        })?;

        render_layers.add_entity(res, &options.position);

        Some(())
    }
}
