use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::{EntityWithOwnerPrototype, WireEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/DisplayPanelPrototype`](https://lua-api.factorio.com/latest/prototypes/DisplayPanelPrototype.html)
pub type DisplayPanelPrototype = EntityWithOwnerPrototype<WireEntityData<DisplayPanelData>>;

/// [`Prototypes/DisplayPanelPrototype`](https://lua-api.factorio.com/latest/prototypes/DisplayPanelPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct DisplayPanelData {
    pub sprites: Option<Sprite4Way>,

    #[serde(
        default = "helper::u32_400",
        skip_serializing_if = "helper::is_400_u32"
    )]
    pub max_text_width: u32,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub text_shift: Vector,

    #[serde(default = "Color::white", skip_serializing_if = "Color::is_white")]
    pub text_color: Color,

    pub background_color: Option<Color>,
}

impl super::Renderable for DisplayPanelData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        if let Some(res) = self.sprites.as_ref().and_then(|s| {
            s.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        }) {
            render_layers.add_entity(res, &options.position);
        }

        // TODO: render icon & text

        Some(())
    }
}
