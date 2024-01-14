use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

mod selection_tool;

pub use selection_tool::*;
use types::{
    Color, FactorioArray, FilterMode, ItemGroupID, ItemID, ItemStackIndex, ItemSubGroupID,
};

/// [`Prototypes/ItemWithLabelPrototype`](https://lua-api.factorio.com/latest/prototypes/ItemWithLabelPrototype.html)
pub type ItemWithLabelPrototype = crate::BasePrototype<ItemWithLabelPrototypeData>;

/// [`Prototypes/ItemWithLabelPrototype`](https://lua-api.factorio.com/latest/prototypes/ItemWithLabelPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ItemWithLabelPrototypeData {
    pub default_label_color: Option<Color>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub draw_label_for_cursor_render: bool,

    #[serde(flatten)]
    parent: super::ItemPrototypeData,
}

impl std::ops::Deref for ItemWithLabelPrototypeData {
    type Target = super::ItemPrototypeData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

/// [`Prototypes/ItemWithInventoryPrototype`](https://lua-api.factorio.com/latest/prototypes/ItemWithInventoryPrototype.html)
pub type ItemWithInventoryPrototype = crate::BasePrototype<ItemWithInventoryPrototypeData>;

/// [`Prototypes/ItemWithInventoryPrototype`](https://lua-api.factorio.com/latest/prototypes/ItemWithInventoryPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ItemWithInventoryPrototypeData {
    // only has a default because BlueprintBookPrototype overrides it
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub inventory_size: ItemStackIndex,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item_filters: FactorioArray<ItemID>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item_group_filters: FactorioArray<ItemGroupID>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item_subgroup_filters: FactorioArray<ItemSubGroupID>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub filter_mode: FilterMode,

    #[serde(
        default = "default_filter_message_key",
        skip_serializing_if = "is_default_filter_message_key"
    )]
    pub filter_message_key: String,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub extends_inventory_by_default: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub insertion_priority_mode: InsertionPriorityMode,

    #[serde(flatten)]
    parent: ItemWithLabelPrototypeData,
    // TODO: stack_size overridden to 1
}

impl std::ops::Deref for ItemWithInventoryPrototypeData {
    type Target = ItemWithLabelPrototypeData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

#[must_use]
fn default_filter_message_key() -> String {
    "item-limitation.item-not-allowed-in-this-container-item".to_owned()
}

#[must_use]
fn is_default_filter_message_key(key: &String) -> bool {
    *key == default_filter_message_key()
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum InsertionPriorityMode {
    #[default]
    Default,
    Never,
    Always,
    WhenManuallyFiltered,
}

// TODO: overrides inventory_size to "dynamic" -.-
/// [`Prototypes/BlueprintBookPrototype`](https://lua-api.factorio.com/latest/prototypes/BlueprintBookPrototype.html)
pub type BlueprintBookPrototype = crate::BasePrototype<BlueprintBookPrototypeData>;

/// [`Prototypes/BlueprintBookPrototype`](https://lua-api.factorio.com/latest/prototypes/BlueprintBookPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct BlueprintBookPrototypeData {
    // inventory_size overridden to "dynamic" or ItemStackIndex
    pub inventory_size: BlueprintBookPrototypeInventorySize,

    #[serde(flatten)]
    parent: ItemWithInventoryPrototypeData,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BlueprintBookPrototypeInventorySize {
    Dynamic(String),
    Numeric(ItemStackIndex),
}

impl std::ops::Deref for BlueprintBookPrototypeData {
    type Target = ItemWithInventoryPrototypeData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

/// [`Prototypes/ItemWithTagsPrototype`](https://lua-api.factorio.com/latest/prototypes/ItemWithTagsPrototype.html)
pub type ItemWithTagsPrototype = ItemWithLabelPrototype;
