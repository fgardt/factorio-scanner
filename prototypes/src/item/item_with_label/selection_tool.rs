use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use types::{
    Color, CursorBoxType, EntityID, FactorioArray, FilterMode, ItemStackIndex, MouseCursorID,
    SelectionModeFlags, TileID,
};

/// [`Prototypes/SelectionToolPrototype`](https://lua-api.factorio.com/latest/prototypes/SelectionToolPrototype.html)
pub type SelectionToolPrototype = crate::BasePrototype<SelectionToolPrototypeData>;

/// [`Prototypes/SelectionToolPrototype`](https://lua-api.factorio.com/latest/prototypes/SelectionToolPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct SelectionToolPrototypeData {
    pub select: SelectionModeData,
    pub alt_select: SelectionModeData,
    pub super_forced_select: Option<SelectionModeData>,
    pub reverse_select: Option<SelectionModeData>,
    pub alt_reverse_select: Option<SelectionModeData>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub always_include_tiles: bool,

    #[serde(
        default = "default_mouse_cursor",
        skip_serializing_if = "is_default_mouse_cursor"
    )]
    pub mouse_cursor: MouseCursorID,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub skip_fog_of_war: bool,

    #[serde(flatten)]
    parent: super::ItemWithLabelPrototypeData,
}

impl std::ops::Deref for SelectionToolPrototypeData {
    type Target = super::ItemWithLabelPrototypeData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

/// [`Types/SelectionModeData`](https://lua-api.factorio.com/latest/types/SelectionModeData.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct SelectionModeData {
    pub border_color: Color,
    pub count_button_color: Option<Color>,
    pub chart_color: Option<Color>,

    pub cursor_box_type: CursorBoxType,
    pub mode: SelectionModeFlags,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entity_filters: FactorioArray<EntityID>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entity_type_filters: FactorioArray<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tile_filters: FactorioArray<TileID>,

    // pub started_sound: Option<Sound>,
    // pub ended_sound: Option<Sound>,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub play_ended_sound_when_nothing_selected: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub entity_filter_mode: FilterMode,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub tile_filter_mode: FilterMode,
}

#[must_use]
fn default_mouse_cursor() -> MouseCursorID {
    MouseCursorID::new("selection-tool-cursor")
}

#[must_use]
fn is_default_mouse_cursor(mouse_cursor: &MouseCursorID) -> bool {
    *mouse_cursor == default_mouse_cursor()
}

// BlueprintItem = SelectionTool
// using newtype pattern to avoid type collisions
/// [`Prototypes/BlueprintItemPrototype`](https://lua-api.factorio.com/latest/prototypes/BlueprintItemPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct BlueprintItemPrototype(SelectionToolPrototype);

impl std::ops::Deref for BlueprintItemPrototype {
    type Target = SelectionToolPrototype;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// [`Prototypes/CopyPasteToolPrototype`](https://lua-api.factorio.com/latest/prototypes/CopyPasteToolPrototype.html)
pub type CopyPasteToolPrototype = crate::BasePrototype<CopyPasteToolPrototypeData>;

/// [`Prototypes/CopyPasteToolPrototype`](https://lua-api.factorio.com/latest/prototypes/CopyPasteToolPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct CopyPasteToolPrototypeData {
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub cuts: bool,

    #[serde(flatten)]
    parent: SelectionToolPrototypeData,
}

impl std::ops::Deref for CopyPasteToolPrototypeData {
    type Target = SelectionToolPrototypeData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

/// [`Prototypes/DeconstructionItemPrototype`](https://lua-api.factorio.com/latest/prototypes/DeconstructionItemPrototype.html)
pub type DeconstructionItemPrototype = crate::BasePrototype<DeconstructionItemPrototypeData>;

/// [`Prototypes/DeconstructionItemPrototype`](https://lua-api.factorio.com/latest/prototypes/DeconstructionItemPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct DeconstructionItemPrototypeData {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub entity_filter_count: ItemStackIndex,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub tile_filter_count: ItemStackIndex,

    #[serde(flatten)]
    parent: SelectionToolPrototypeData,
}

impl std::ops::Deref for DeconstructionItemPrototypeData {
    type Target = SelectionToolPrototypeData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

/// [`Prototypes/SpidertronRemotePrototype`](https://lua-api.factorio.com/latest/prototypes/SpidertronRemotePrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct SpidertronRemotePrototype(SelectionToolPrototype);

impl std::ops::Deref for SpidertronRemotePrototype {
    type Target = SelectionToolPrototype;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// [`Prototypes/UpgradeItemPrototype`](https://lua-api.factorio.com/latest/prototypes/UpgradeItemPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct UpgradeItemPrototype(SelectionToolPrototype);

impl std::ops::Deref for UpgradeItemPrototype {
    type Target = SelectionToolPrototype;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
