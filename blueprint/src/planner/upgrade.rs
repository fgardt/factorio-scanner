use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::IndexedVec;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type", deny_unknown_fields)]
pub enum MappedValue {
    Entity { name: String },
    Item { name: String },
}

impl MappedValue {
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Entity { name } | Self::Item { name } => name,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MappingEntry {
    pub from: Option<MappedValue>,
    pub to: Option<MappedValue>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpgradePlannerData {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mappers: IndexedVec<MappingEntry>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub icons: IndexedVec<crate::Icon>,
}

// not a correct implementation, but its good enough for serialization skipping when default
impl PartialEq for UpgradePlannerData {
    fn eq(&self, other: &Self) -> bool {
        self.description == other.description
            && self.mappers.len() == other.mappers.len()
            && self.icons.len() == other.icons.len()
    }
}

pub type UpgradePlanner = crate::CommonData<super::PlannerData<UpgradePlannerData>>;
