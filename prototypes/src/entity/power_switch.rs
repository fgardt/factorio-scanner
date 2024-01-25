use serde::{Deserialize, Serialize};

use serde_helper as helper;

use super::{EntityWithOwnerPrototype, WireEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/PowerSwitchPrototype`](https://lua-api.factorio.com/latest/prototypes/PowerSwitchPrototype.html)
pub type PowerSwitchPrototype = EntityWithOwnerPrototype<WireEntityData<PowerSwitchData>>;

/// [`Prototypes/PowerSwitchPrototype`](https://lua-api.factorio.com/latest/prototypes/PowerSwitchPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct PowerSwitchData {
    pub power_on_animation: Animation,
    pub overlay_start: Animation,
    pub overlay_loop: Animation,
    pub led_on: Sprite,
    pub led_off: Sprite,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub overlay_start_delay: u8,
}

impl super::Renderable for PowerSwitchData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self.power_on_animation.render(
            render_layers.scale(),
            used_mods,
            image_cache,
            &options.into(),
        )?;

        render_layers.add_entity(res, &options.position);

        Some(())

        // TODO: render open / closed depending on render option flag
    }
}
