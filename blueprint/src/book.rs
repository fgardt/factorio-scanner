use serde::{Deserialize, Serialize};

use crate::IndexedVec;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BookData {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub icons: IndexedVec<crate::Icon>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blueprints: IndexedVec<Box<crate::Data>>,

    pub active_index: u16,
}

impl crate::GetIDs for BookData {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = self.icons.get_ids();

        ids.merge(self.blueprints.get_ids());

        ids
    }
}

pub type Book = crate::CommonData<BookData>;
