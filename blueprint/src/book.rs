use serde::{Deserialize, Serialize};

use crate::IndexedVec;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookData {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub icons: IndexedVec<crate::Icon>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blueprints: IndexedVec<Box<crate::Data>>,

    pub active_index: u16,
}

pub type Book = crate::CommonData<BookData>;
