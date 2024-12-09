use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use types::{Animation, BoxSpecification, EntityBuildAnimationPiece, FactorioArray, Sprite};

/// [`Prototypes/UtilitySprites`](https://lua-api.factorio.com/latest/prototypes/UtilitySprites.html)
pub type UtilitySprites = crate::BasePrototype<UtilitySpritesData>;

/// [`Prototypes/UtilitySprites`](https://lua-api.factorio.com/latest/prototypes/UtilitySprites.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct UtilitySpritesData {
    pub cursor_box: CursorBoxSpecification,

    pub platform_entity_build_animations: Option<EntityBuildAnimations>,

    pub clouds: Animation,
    pub arrow_button: Animation,
    pub explosion_chart_visualization: Animation,
    pub refresh_white: Animation,
    pub navmesh_pending_icon: Animation,

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
    pub multiplayer_selection: FactorioArray<BoxSpecification>,
    pub not_allowed: FactorioArray<BoxSpecification>,
    pub copy: FactorioArray<BoxSpecification>,
    pub electricity: FactorioArray<BoxSpecification>,
    pub logistics: FactorioArray<BoxSpecification>,
    pub pair: FactorioArray<BoxSpecification>,
    pub train_visualization: FactorioArray<BoxSpecification>,
    pub blueprint_snap_rectangle: FactorioArray<BoxSpecification>,
    pub spidertron_remote_selected: FactorioArray<BoxSpecification>,
    pub spidertron_remote_to_be_selected: FactorioArray<BoxSpecification>,
}

/// [`Prototypes/UtilitySprites/EntityBuildAnimations`](https://lua-api.factorio.com/latest/prototypes/UtilitySprites.html#platform_entity_build_animations)
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityBuildAnimations {
    pub back_left: EntityBuildAnimationPiece,
    pub back_right: EntityBuildAnimationPiece,
    pub front_left: EntityBuildAnimationPiece,
    pub front_right: EntityBuildAnimationPiece,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WireSprites {
    pub copper_wire: Sprite,
    pub green_wire: Sprite,
    pub red_wire: Sprite,

    pub copper_wire_highlight: Sprite,
    pub green_wire_highlight: Sprite,
    pub red_wire_highlight: Sprite,

    pub wire_shadow: Sprite,
}
