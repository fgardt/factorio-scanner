use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_with::skip_serializing_none;
use types::{Comparator, ItemCountType, QualityID, SpaceLocationID};

use crate::IndexedVec;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LogisticSections {
    pub sections: IndexedVec<LogisticSection>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub trash_not_requested: bool,
}

impl crate::GetIDs for LogisticSections {
    fn get_ids(&self) -> crate::UsedIDs {
        self.sections.get_ids()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RequestFilters {
    pub sections: IndexedVec<LogisticSection>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub request_from_buffers: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub trash_not_requested: bool,
}

impl crate::GetIDs for RequestFilters {
    fn get_ids(&self) -> crate::UsedIDs {
        self.sections.get_ids()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[skip_serializing_none]
pub struct LogisticSection {
    #[serde(
        default = "serde_helper::bool_true",
        skip_serializing_if = "Clone::clone"
    )]
    pub active: bool,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: IndexedVec<LogisticFilter>,
    pub group: Option<String>,

    #[serde(
        default = "serde_helper::f64_1",
        skip_serializing_if = "serde_helper::is_1_f64"
    )]
    pub multiplier: f64,
}

impl Eq for LogisticSection {}

impl crate::GetIDs for LogisticSection {
    fn get_ids(&self) -> crate::UsedIDs {
        self.filters.get_ids()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
#[skip_serializing_none]
pub struct LogisticFilter {
    // #[serde(flatten)]
    // pub signal: SignalID,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub name: Option<String>,
    pub quality: Option<QualityID>,

    pub comparator: Option<Comparator>,
    pub count: i32,
    pub max_count: Option<ItemCountType>,
    pub minimum_delivery_count: Option<ItemCountType>,
    pub import_from: Option<SpaceLocationID>,
}

impl crate::GetIDs for LogisticFilter {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = crate::UsedIDs::default();

        if let Some(import_from) = &self.import_from {
            ids.space_location.insert(import_from.clone());
        }

        ids
    }
}
