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

impl std::fmt::Display for AnyBasic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "{s}"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::Number(n) => write!(f, "{n}"),
            Self::Table(t) => {
                write!(f, "{{")?;
                for (i, (k, v)) in t.iter().enumerate() {
                    if i != 0 && i != t.len() {
                        write!(f, ", ")?;
                    }

                    write!(f, "{k}: {v}")?;
                }
                write!(f, "}}")
            }
            Self::Array(a) => {
                write!(f, "[")?;
                for (i, e) in a.iter().enumerate() {
                    if i != 0 && i != a.len() {
                        write!(f, ", ")?;
                    }

                    write!(f, "{e}")?;
                }
                write!(f, "]")
            }
        }
    }
}
