use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::EntityWithOwnerPrototype;
use crate::data_raw::types::*;
/// [`Prototypes/HeatInterfacePrototype`](https://lua-api.factorio.com/latest/prototypes/HeatInterfacePrototype.html)
pub type HeatInterfacePrototype = EntityWithOwnerPrototype<HeatInterfaceData>;

/// [`Prototypes/HeatInterfacePrototype`](https://lua-api.factorio.com/latest/prototypes/HeatInterfacePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct HeatInterfaceData {
    pub heat_buffer: HeatBuffer,

    pub picture: Option<Sprite>,

    #[serde(default = "GuiMode::all", skip_serializing_if = "GuiMode::is_all")]
    pub gui_mode: GuiMode,
}
