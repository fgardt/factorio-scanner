use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_with::skip_serializing_none;

use types::{Comparator, EquipmentID, ItemCountType, ItemID, ItemStackIndex, QualityID};

use crate::IndexedVec;

/// [`BlueprintInsertPlan`](https://lua-api.factorio.com/latest/concepts/BlueprintInsertPlan.html)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct InsertPlan {
    pub id: TAndQualityIDPair<ItemID>,
    pub items: ItemInventoryPositions,
}

impl crate::GetIDs for InsertPlan {
    fn get_ids(&self) -> crate::UsedIDs {
        self.id.get_ids()
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TAndQualityIDPair<T: crate::GetIDs> {
    pub name: T,

    #[serde(
        default = "QualityID::normal",
        skip_serializing_if = "QualityID::is_normal"
    )]
    pub quality: QualityID,
}

impl<T: crate::GetIDs> crate::GetIDs for TAndQualityIDPair<T> {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = self.name.get_ids();
        ids.quality.insert(self.quality.clone());

        ids
    }
}

/// [`ItemInventoryPositions`](https://lua-api.factorio.com/latest/concepts/ItemInventoryPositions.html)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ItemInventoryPositions {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub in_inventory: Vec<InventoryPosition>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub grid_count: ItemCountType,
}

/// [`InventoryPosition`](https://lua-api.factorio.com/latest/concepts/InventoryPosition.html)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct InventoryPosition {
    pub inventory: ItemStackIndex, // should be defines.inventory
    pub stack: ItemStackIndex,

    #[serde(default = "helper::u32_1", skip_serializing_if = "helper::is_1_u32")]
    pub count: ItemCountType,
}

/// [`BlueprintInventoryWithFilters`](https://lua-api.factorio.com/latest/concepts/BlueprintInventoryWithFilters.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct InventoryWithFilters {
    pub bar: Option<ItemStackIndex>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: IndexedVec<ItemFilter>,
}

impl crate::GetIDs for InventoryWithFilters {
    fn get_ids(&self) -> crate::UsedIDs {
        self.filters.get_ids()
    }
}

/// [`BlueprintItemFilter`](https://lua-api.factorio.com/latest/concepts/BlueprintItemFilter.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ItemFilter {
    pub name: Option<ItemID>,
    pub quality: Option<QualityID>,
    pub comparator: Option<Comparator>,
}

impl crate::GetIDs for ItemFilter {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = self.name.get_ids();
        ids.merge(self.quality.get_ids());

        ids
    }
}

/// [`BlueprintInfinityInventorySettings`](https://lua-api.factorio.com/latest/concepts/BlueprintInfinityInventorySettings.html)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct InfinityInventorySettings {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: IndexedVec<InfinityInventoryFilter>,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub remove_unfiltered_items: bool,
}

impl crate::GetIDs for InfinityInventorySettings {
    fn get_ids(&self) -> crate::UsedIDs {
        self.filters.get_ids()
    }
}

/// [`InfinityInventoryFilter`](https://lua-api.factorio.com/latest/concepts/InfinityInventoryFilter.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct InfinityInventoryFilter {
    pub name: ItemID,
    #[serde(
        default = "QualityID::normal",
        skip_serializing_if = "QualityID::is_normal"
    )]
    pub quality: QualityID,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub count: ItemCountType,
    pub mode: Option<InfinityInventoryFilterMode>,
}

impl crate::GetIDs for InfinityInventoryFilter {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = crate::UsedIDs::default();
        ids.item.insert(self.name.clone());
        ids.quality.insert(self.quality.clone());

        ids
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum InfinityInventoryFilterMode {
    AtLeast,
    AtMost,
    Exactly,
}

/// [`BlueprintEquipment`](https://lua-api.factorio.com/latest/concepts/BlueprintEquipment.html)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct BlueprintEquipment {
    pub equipment: TAndQualityIDPair<EquipmentID>,
    pub position: EquipmentPosition,
}

impl crate::GetIDs for BlueprintEquipment {
    fn get_ids(&self) -> crate::UsedIDs {
        self.equipment.get_ids()
    }
}

/// [`EquipmentPosition`](https://lua-api.factorio.com/latest/concepts/EquipmentPosition.html)
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum EquipmentPosition {
    Tuple(i32, i32),
    Table { x: i32, y: i32 },
}

impl EquipmentPosition {
    #[must_use]
    pub const fn as_tuple(&self) -> (i32, i32) {
        match self {
            Self::Tuple(x, y) | Self::Table { x, y } => (*x, *y),
        }
    }
}

impl PartialEq for EquipmentPosition {
    fn eq(&self, other: &Self) -> bool {
        self.as_tuple() == other.as_tuple()
    }
}

impl Eq for EquipmentPosition {}
