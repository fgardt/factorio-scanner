use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
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
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub direction_count: DirCount,
    pub picture: Option<Sprite4Way>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_inventory_content: bool,

    pub default_empty_slots_signal: Option<SignalIDConnector>,
}

impl super::Entity for ProxyContainerData {}

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

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(u8)]
pub enum DirCount {
    #[default]
    One = 1,
    Two = 2,
    Four = 4,
}
