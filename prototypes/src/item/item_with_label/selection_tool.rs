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
    pub selection_mode: SelectionModeFlags,
    pub alt_selection_mode: SelectionModeFlags,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub always_include_tiles: bool,

    pub selection_color: Color,
    pub alt_selection_color: Color,
    pub selection_cursor_box_type: CursorBoxType,
    pub alt_selection_cursor_box_type: CursorBoxType,

    // defaults depending on values of other fields
    pub reverse_selection_color: Option<Color>,
    pub alt_reverse_selection_color: Option<Color>,
    pub selection_count_button_color: Option<Color>,

    pub alt_selection_count_button_color: Option<Color>,
    pub reverse_selection_count_button_color: Option<Color>,
    pub alt_reverse_selection_count_button_color: Option<Color>,

    pub chart_selection_color: Option<Color>,
    pub chart_alt_selection_color: Option<Color>,
    pub chart_reverse_selection_color: Option<Color>,
    pub chart_alt_reverse_selection_color: Option<Color>,

    pub reverse_selection_mode: Option<SelectionModeFlags>,
    pub alt_reverse_selection_mode: Option<SelectionModeFlags>,
    pub reverse_selection_cursor_box_type: Option<CursorBoxType>,
    pub alt_reverse_selection_cursor_box_type: Option<CursorBoxType>,

    #[serde(
        default = "default_mouse_cursor",
        skip_serializing_if = "is_default_mouse_cursor"
    )]
    pub mouse_cursor: MouseCursorID,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entity_filters: FactorioArray<EntityID>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alt_entity_filters: FactorioArray<EntityID>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entity_type_filters: FactorioArray<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alt_entity_type_filters: FactorioArray<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tile_filters: FactorioArray<TileID>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alt_tile_filters: FactorioArray<TileID>,

    // TODO: skip serializing if default
    #[serde(default)]
    pub entity_filter_mode: FilterMode,
    #[serde(default)]
    pub alt_entity_filter_mode: FilterMode,
    #[serde(default)]
    pub tile_filter_mode: FilterMode,
    #[serde(default)]
    pub alt_tile_filter_mode: FilterMode,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reverse_entity_filters: FactorioArray<EntityID>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alt_reverse_entity_filters: FactorioArray<EntityID>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reverse_entity_type_filters: FactorioArray<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alt_reverse_entity_type_filters: FactorioArray<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reverse_tile_filters: FactorioArray<TileID>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alt_reverse_tile_filters: FactorioArray<TileID>,

    // TODO: skip serializing if default
    #[serde(default)]
    pub reverse_entity_filter_mode: FilterMode,
    #[serde(default)]
    pub alt_reverse_entity_filter_mode: FilterMode,
    #[serde(default)]
    pub reverse_tile_filter_mode: FilterMode,
    #[serde(default)]
    pub alt_reverse_tile_filter_mode: FilterMode,

    #[serde(flatten)]
    parent: super::ItemWithLabelPrototypeData,
}

impl std::ops::Deref for SelectionToolPrototypeData {
    type Target = super::ItemWithLabelPrototypeData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

#[must_use]
fn default_mouse_cursor() -> MouseCursorID {
    "selection-tool-cursor".to_owned()
}

#[must_use]
fn is_default_mouse_cursor(mouse_cursor: &MouseCursorID) -> bool {
    *mouse_cursor == default_mouse_cursor()
}

// [`Prototypes/BlueprintItemPrototype`](https://lua-api.factorio.com/latest/prototypes/BlueprintItemPrototype.html)
pub type BlueprintItemPrototype = SelectionToolPrototype;

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
    #[serde(default, skip_serializing_if = "helper::is_0_u16")]
    pub entity_filter_count: ItemStackIndex,

    #[serde(default, skip_serializing_if = "helper::is_0_u16")]
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

/// [`Prototypes/UpgradeItemPrototype`](https://lua-api.factorio.com/latest/prototypes/UpgradeItemPrototype.html)
pub type UpgradeItemPrototype = crate::BasePrototype<UpgradeItemPrototypeData>;

/// [`Prototypes/UpgradeItemPrototype`](https://lua-api.factorio.com/latest/prototypes/UpgradeItemPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct UpgradeItemPrototypeData {
    #[serde(default, skip_serializing_if = "helper::is_0_u16")]
    pub mapper_count: ItemStackIndex,

    #[serde(flatten)]
    parent: SelectionToolPrototypeData,
}

impl std::ops::Deref for UpgradeItemPrototypeData {
    type Target = SelectionToolPrototypeData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
