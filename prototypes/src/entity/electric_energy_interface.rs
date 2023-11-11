use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/ElectricEnergyInterfacePrototype`](https://lua-api.factorio.com/latest/prototypes/ElectricEnergyInterfacePrototype.html)
pub type ElectricEnergyInterfacePrototype = EntityWithOwnerPrototype<ElectricEnergyInterfaceData>;

/// [`Prototypes/ElectricEnergyInterfacePrototype`](https://lua-api.factorio.com/latest/prototypes/ElectricEnergyInterfacePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ElectricEnergyInterfaceData {
    pub energy_source: ElectricEnergySource,

    pub energy_production: Option<Energy>,
    pub energy_usage: Option<Energy>,

    #[serde(default = "GuiMode::none", skip_serializing_if = "GuiMode::is_none")]
    pub gui_mode: GuiMode,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub continuous_animation: bool,

    // TODO: skip serializing if default
    pub render_layer: Option<RenderLayer>,

    pub light: Option<LightDefinition>,

    #[serde(flatten)]
    pub graphics: Option<ElectricEnergyInterfaceGraphics>,
    // TODO: `allow_copy_paste` has overriden default
}

impl super::Renderable for ElectricEnergyInterfaceData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        target_size: &TargetSize,
        render_layers: &mut RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.graphics
            .as_ref()?
            .render(options, used_mods, target_size, render_layers, image_cache)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ElectricEnergyInterfaceGraphics {
    Picture { picture: Sprite },
    Pictures { pictures: Sprite4Way },
    Animation { animation: Animation },
    Animations { animations: Animation4Way },
}

impl super::Renderable for ElectricEnergyInterfaceGraphics {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        target_size: &TargetSize,
        render_layers: &mut RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        match self {
            Self::Picture { picture } => picture.render(used_mods, image_cache, &options.into()),
            Self::Pictures { pictures } => pictures.render(used_mods, image_cache, &options.into()),
            Self::Animation { animation } => {
                animation.render(used_mods, image_cache, &options.into())
            }
            Self::Animations { animations } => {
                animations.render(used_mods, image_cache, &options.into())
            }
        }
    }
}
