use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// <https://lua-api.factorio.com/latest/concepts.html#Tags>
pub type TagTable = HashMap<String, AnyBasic>;

/// <https://lua-api.factorio.com/latest/concepts.html#AnyBasic>
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(untagged, deny_unknown_fields)]
pub enum AnyBasic {
    String(String),
    Bool(bool),
    Number(f64),
    Table(TagTable),
    Array(Vec<AnyBasic>),
}
