use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use types::{Animation, BoxSpecification, FactorioArray, Sprite};

/// [`Prototypes/UtilitySprites`](https://lua-api.factorio.com/latest/prototypes/UtilitySprites.html)
pub type UtilitySprites = crate::BasePrototype<UtilitySpritesData>;

/// [`Prototypes/UtilitySprites`](https://lua-api.factorio.com/latest/prototypes/UtilitySprites.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct UtilitySpritesData {
    pub cursor_box: CursorBoxSpecification,
    pub clouds: Animation,
    pub arrow_button: Animation,
    pub explosion_chart_visualization: Animation,
    pub refresh_white: Animation,

    #[serde(flatten)]
    pub wires: WireSprites,

    pub indication_arrow: Sprite,
    pub indication_line: Sprite,
    pub short_indication_line: Sprite,

    #[serde(flatten)]
    pub sprites: HashMap<String, Sprite>,
}

/// [`Prototypes/UtilitySprites/CursorBoxSpecification`](https://lua-api.factorio.com/latest/prototypes/UtilitySprites.html#cursor_box)
#[derive(Debug, Serialize, Deserialize)]
pub struct CursorBoxSpecification {
    pub regular: FactorioArray<BoxSpecification>,
    pub not_allowed: FactorioArray<BoxSpecification>,
    pub copy: FactorioArray<BoxSpecification>,
    pub electricity: FactorioArray<BoxSpecification>,
    pub logistics: FactorioArray<BoxSpecification>,
    pub pair: FactorioArray<BoxSpecification>,
    pub train_visualization: FactorioArray<BoxSpecification>,
    pub blueprint_snap_rectangle: FactorioArray<BoxSpecification>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WireSprites {
    pub copper_wire: Sprite,
    pub green_wire: Sprite,
    pub red_wire: Sprite,

    pub wire_shadow: Sprite,

    #[serde(rename = "green_wire_hightlight", alias = "green_wire_highlight")]
    pub green_wire_highlight: Sprite,
    #[serde(rename = "red_wire_hightlight", alias = "red_wire_highlight")]
    pub red_wire_highlight: Sprite,
}
