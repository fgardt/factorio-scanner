use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_with::skip_serializing_none;
use types::{Comparator, ItemCountType, QualityID, SpaceLocationID};

use crate::{IndexedVec, SignalIDType};

/// [`BlueprintLogisticSections`](https://lua-api.factorio.com/latest/concepts/BlueprintLogisticSections.html)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LogisticSections {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sections: IndexedVec<LogisticSection>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub trash_not_requested: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub request_from_buffers: bool,
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

/// [`LogisticSection`](https://lua-api.factorio.com/latest/concepts/LogisticSection.html)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[skip_serializing_none]
pub struct LogisticSection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: IndexedVec<LogisticFilter>,

    pub group: Option<String>,

    #[serde(
        default = "serde_helper::f64_1",
        skip_serializing_if = "serde_helper::is_1_f64"
    )]
    pub multiplier: f64,

    #[serde(
        default = "serde_helper::bool_true",
        skip_serializing_if = "Clone::clone"
    )]
    pub active: bool,
}

impl Eq for LogisticSection {}

impl crate::GetIDs for LogisticSection {
    fn get_ids(&self) -> crate::UsedIDs {
        self.filters.get_ids()
    }
}

/// [`BlueprintLogisticFilter`](https://lua-api.factorio.com/latest/concepts/BlueprintLogisticFilter.html)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
#[skip_serializing_none]
pub struct LogisticFilter {
    #[serde(rename = "type")]
    pub kind: Option<SignalIDType>,
    pub name: Option<String>,
    pub quality: Option<QualityID>,

    pub comparator: Option<Comparator>,
    pub count: i32,
    pub max_count: Option<ItemCountType>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub minimum_delivery_count: ItemCountType,
    pub import_from: Option<SpaceLocationID>,
}

impl crate::GetIDs for LogisticFilter {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = crate::UsedIDs::default();

        if let Some(kind) = &self.kind
            && let Some(name) = self.name.clone()
        {
            match kind {
                SignalIDType::Item => ids.item.insert(name.into()),
                SignalIDType::Fluid => ids.fluid.insert(name.into()),
                SignalIDType::Virtual => ids.virtual_signal.insert(name.into()),
                SignalIDType::Entity => ids.entity.insert(name.into()),
                SignalIDType::Recipe => ids.recipe.insert(name.into()),
                SignalIDType::SpaceLocation => ids.space_location.insert(name.into()),
                SignalIDType::AsteroidChunk => ids.asteroid_chunk.insert(name.into()),
                SignalIDType::Quality => ids.quality.insert(name.into()),
            };
        }

        if let Some(quality) = &self.quality {
            ids.quality.insert(quality.clone());
        }

        if let Some(import_from) = &self.import_from {
            ids.space_location.insert(import_from.clone());
        }

        ids
    }
}
