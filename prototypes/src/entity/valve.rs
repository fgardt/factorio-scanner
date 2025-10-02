use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_with::skip_serializing_none;

use super::{EntityWithOwnerPrototype, abstractions::FluidBoxEntityData};
use types::*;

/// [`Prototypes/Valve`](https://lua-api.factorio.com/latest/prototypes/ValvePrototype.html)
pub type ValvePrototype = EntityWithOwnerPrototype<FluidBoxEntityData<ValveData>>;

/// [`Prototypes/Valve`](https://lua-api.factorio.com/latest/prototypes/ValvePrototype.html)
#[derive(Debug, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct ValveData {
    pub mode: ValveMode,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub threshold: f32,

    pub flow_rate: FluidAmount,
    pub animations: Option<Animation4Way>,
    pub front_patch: Option<Sprite4Way>,
}

/// [`Types/ValveMode`](https://lua-api.factorio.com/latest/types/ValveMode.html)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ValveMode {
    OneWay,
    Overflow,
    TopUp,
}

impl super::Renderable for ValveData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &mod_util::UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self.animations.as_ref().and_then(|a| {
            a.render(
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
