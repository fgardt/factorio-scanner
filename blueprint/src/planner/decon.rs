use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{IndexedVec, NameString};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum FilterMode {
    #[default]
    Whitelist = 0,
    Blacklist = 1,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TileSelectionMode {
    #[default]
    Normal = 0,
    Always = 1,
    Never = 2,
    Only = 3,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeconPlannerData {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub entity_filter_mode: FilterMode,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entity_filters: IndexedVec<NameString>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub trees_and_rocks_only: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub tile_filter_mode: FilterMode,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub tile_selection_mode: TileSelectionMode,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tile_filters: IndexedVec<NameString>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub icons: IndexedVec<crate::Icon>,
}

impl PartialEq for DeconPlannerData {
    fn eq(&self, other: &Self) -> bool {
        self.entity_filter_mode == other.entity_filter_mode
            && self.entity_filters.len() == other.entity_filters.len()
            && self.trees_and_rocks_only == other.trees_and_rocks_only
            && self.tile_filter_mode == other.tile_filter_mode
            && self.tile_selection_mode == other.tile_selection_mode
            && self.tile_filters.len() == other.tile_filters.len()
            && self.description == other.description
            && self.icons.len() == other.icons.len()
    }
}

pub type DeconPlanner = crate::CommonData<super::PlannerData<DeconPlannerData>>;
