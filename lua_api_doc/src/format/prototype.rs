use std::io::Read;

use diff::Diff;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff)]
#[diff(attr(
    #[derive(Debug)]
))]
pub struct PrototypeDoc {
    #[serde(flatten)]
    pub common: super::Common,

    pub prototypes: Vec<Prototype>,
    pub types: Vec<TypeConcept>,
}

impl PrototypeDoc {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut bytes = Vec::new();
        std::fs::File::open(path).unwrap().read_to_end(&mut bytes)?;
        let doc = serde_json::from_slice(&bytes)?;
        Ok(doc)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff)]
#[diff(attr(
    #[derive(Debug)]
))]
pub struct Common {
    pub description: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lists: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<Image>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff)]
#[diff(attr(
    #[derive(Debug)]
))]
pub struct NamedCommon {
    #[serde(flatten)]
    pub common: Common,

    pub name: String,
    pub order: i16,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff)]
#[diff(attr(
    #[derive(Debug)]
))]
pub struct Prototype {
    #[serde(flatten)]
    pub common: NamedCommon,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub parent: String,

    #[serde(rename = "abstract")]
    pub abstract_: bool,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub typename: String,

    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // pub instance_limit: Option<u128>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub instance_limit: String,

    pub deprecated: bool,

    pub properties: Vec<Property>,
    pub custom_properties: Option<CustomProperties>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff)]
#[diff(attr(
    #[derive(Debug)]
))]
pub struct TypeConcept {
    #[serde(flatten)]
    pub common: NamedCommon,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub parent: String,

    #[serde(rename = "abstract")]
    pub abstract_: bool,

    pub inline: bool,

    #[serde(rename = "type")]
    pub type_: Type,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<Property>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff)]
#[diff(attr(
    #[derive(Debug)]
))]
pub struct Image {
    pub filename: String,
    pub caption: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff)]
#[diff(attr(
    #[derive(Debug)]
))]
pub struct Property {
    #[serde(flatten)]
    pub common: NamedCommon,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub alt_name: String,

    #[serde(rename = "override")]
    pub override_: bool,

    #[serde(rename = "type")]
    pub type_: Type,

    pub optional: bool,
    pub default: Option<PropertyDefault>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff)]
#[diff(attr(
    #[derive(Debug)]
))]
#[serde(untagged)]
pub enum PropertyDefault {
    String(String),
    Literal(Literal),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff)]
#[diff(attr(
    #[derive(Debug)]
))]
pub struct CustomProperties {
    #[serde(flatten)]
    pub common: Common,

    pub key_type: Type,
    pub value_type: Type,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff)]
#[diff(attr(
    #[derive(Debug)]
))]
#[serde(untagged)]
pub enum Type {
    Simple(String),
    Complex(Box<ComplexType>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff)]
#[diff(attr(
    #[derive(Debug)]
))]
#[serde(tag = "complex_type", rename_all = "snake_case")]
pub enum ComplexType {
    Array {
        value: Type,
    },
    Dictionary {
        key: Type,
        value: Type,
    },
    Tuple {
        values: Vec<Type>,
    },
    Union {
        options: Vec<Type>,
        full_format: bool,
    },
    Type {
        value: Type,
        description: String,
    },
    Literal(Literal),
    Struct,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff)]
#[diff(attr(
    #[derive(Debug)]
))]
pub struct Literal {
    pub value: LiteralValue,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff)]
#[diff(attr(
    #[derive(Debug)]
))]
#[serde(untagged)]
pub enum LiteralValue {
    String(String),
    UInt(u64),
    Int(i64),
    Float(f64),
    Boolean(bool),
}
