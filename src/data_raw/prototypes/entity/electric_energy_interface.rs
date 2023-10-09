use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::EntityWithOwnerPrototype;
use crate::data_raw::types::*;

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
    pub graphics: ElectricEnergyInterfaceGraphics,
    // TODO: `allow_copy_paste` has overriden default
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ElectricEnergyInterfaceGraphics {
    Picture { picture: Sprite },
    Pictures { pictures: Sprite4Way },
    Animation { animation: Animation },
    Animations { animations: Animation4Way },
}
