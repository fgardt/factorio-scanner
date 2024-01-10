use std::{io::Read, ops::Deref};

use diff::Diff;
use serde::{Deserialize, Serialize};

use super::DiffPrint;

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff)]
#[diff(attr(
    #[derive(Debug, Serialize, Deserialize)]
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

impl DiffPrint<PrototypeDoc> for PrototypeDocDiff {
    fn diff_print(&self, old: &PrototypeDoc, new: &PrototypeDoc, indent: usize, _name: &str) {
        self.common.diff_print(&old.common, &new.common, indent, "");
        self.prototypes
            .diff_print(&old.prototypes, &new.prototypes, indent, "prototypes");
        self.types
            .diff_print(&old.types, &new.types, indent, "types");
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Diff, Clone, Default)]
#[diff(attr(
    #[derive(Debug, Serialize, Deserialize)]
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

impl DiffPrint<Common> for CommonDiff {
    fn diff_print(&self, old: &Common, new: &Common, indent: usize, _name: &str) {
        self.description
            .diff_print(&old.description, &new.description, indent, "description");

        self.lists
            .diff_print(&old.lists, &new.lists, indent, "lists");

        self.examples
            .diff_print(&old.examples, &new.examples, indent, "examples");

        self.images
            .diff_print(&old.images, &new.images, indent, "images");
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Diff, Clone, Default)]
#[diff(attr(
    #[derive(Debug, Serialize, Deserialize)]
))]
pub struct NamedCommon {
    #[serde(flatten)]
    common: Common,

    pub name: String,
    pub order: i16,
}

impl Deref for NamedCommon {
    type Target = Common;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl Deref for NamedCommonDiff {
    type Target = CommonDiff;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DiffPrint<NamedCommon> for NamedCommonDiff {
    fn diff_print(&self, old: &NamedCommon, new: &NamedCommon, indent: usize, _name: &str) {
        self.name.diff_print(&old.name, &new.name, indent, "name");
        self.order
            .diff_print(&old.order, &new.order, indent, "order");
        self.common.diff_print(&old.common, &new.common, indent, "");
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff)]
#[diff(attr(
    #[derive(Debug, Serialize, Deserialize)]
))]
pub struct Prototype {
    #[serde(flatten)]
    common: NamedCommon,

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

impl Deref for Prototype {
    type Target = NamedCommon;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl Deref for PrototypeDiff {
    type Target = NamedCommonDiff;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DiffPrint<Prototype> for PrototypeDiff {
    fn diff_print(&self, old: &Prototype, new: &Prototype, indent: usize, _name: &str) {
        self.common.diff_print(&old.common, &new.common, indent, "");
        self.parent
            .diff_print(&old.parent, &new.parent, indent, "parent");
        self.abstract_
            .diff_print(&old.abstract_, &new.abstract_, indent, "abstract");
        self.typename
            .diff_print(&old.typename, &new.typename, indent, "typename");
        self.instance_limit.diff_print(
            &old.instance_limit,
            &new.instance_limit,
            indent,
            "instance_limit",
        );
        self.deprecated
            .diff_print(&old.deprecated, &new.deprecated, indent, "deprecated");
        self.properties
            .diff_print(&old.properties, &new.properties, indent, "properties");
        self.custom_properties.diff_print(
            &old.custom_properties,
            &new.custom_properties,
            indent,
            "custom_properties",
        );
    }
}

impl DiffPrint<Vec<Prototype>> for diff::VecDiff<Prototype> {
    fn diff_print(&self, old: &Vec<Prototype>, new: &Vec<Prototype>, indent: usize, name: &str) {
        let indent_str = " ".repeat(indent);
        if self.0.is_empty() {
            return;
        }

        println!("{indent_str}{name}:");
        for diff in &self.0 {
            match diff {
                diff::VecDiffType::Inserted { index, .. } => {
                    println!("{indent_str}  +{}", new[*index].name);
                }
                diff::VecDiffType::Removed { index, .. } => {
                    println!("{indent_str}  -{}", old[*index].name);
                }
                diff::VecDiffType::Altered { index, changes } => {
                    println!("{indent_str}  *{}", &old[*index].name);
                    for diff in changes {
                        diff.diff_print(&old[*index], &new[*index], indent + 4, "");
                    }
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff, Clone, Default)]
#[diff(attr(
    #[derive(Debug, Serialize, Deserialize)]
))]
pub struct TypeConcept {
    #[serde(flatten)]
    common: NamedCommon,

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

impl Deref for TypeConcept {
    type Target = NamedCommon;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl Deref for TypeConceptDiff {
    type Target = NamedCommonDiff;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DiffPrint<TypeConcept> for TypeConceptDiff {
    fn diff_print(&self, old: &TypeConcept, new: &TypeConcept, indent: usize, _name: &str) {
        self.common.diff_print(&old.common, &new.common, indent, "");
        self.parent
            .diff_print(&old.parent, &new.parent, indent, "parent");
        self.abstract_
            .diff_print(&old.abstract_, &new.abstract_, indent, "abstract");
        self.inline
            .diff_print(&old.inline, &new.inline, indent, "inline");
        self.type_
            .diff_print(&old.type_, &new.type_, indent, "type");
        self.properties
            .diff_print(&old.properties, &new.properties, indent, "properties");
    }
}

impl DiffPrint<Vec<TypeConcept>> for diff::VecDiff<TypeConcept> {
    fn diff_print(
        &self,
        old: &Vec<TypeConcept>,
        new: &Vec<TypeConcept>,
        indent: usize,
        name: &str,
    ) {
        let indent_str = " ".repeat(indent);
        if self.0.is_empty() {
            return;
        }

        println!("{indent_str}{name}:");
        for diff in &self.0 {
            match diff {
                diff::VecDiffType::Inserted { index, .. } => {
                    println!(
                        "{indent_str}  +{}",
                        new.get(*index).cloned().unwrap_or_default().name
                    );
                }
                diff::VecDiffType::Removed { index, .. } => {
                    println!(
                        "{indent_str}  -{}",
                        old.get(*index).cloned().unwrap_or_default().name
                    );
                }
                diff::VecDiffType::Altered { index, changes } => {
                    let (o, n) = if old.get(*index).is_none() || new.get(*index).is_none() {
                        (TypeConcept::default(), TypeConcept::default())
                    } else {
                        (old[*index].clone(), new[*index].clone())
                    };

                    println!("{indent_str}  *{}", n.name);
                    for diff in changes {
                        diff.diff_print(&o, &n, indent + 4, "");
                    }
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Diff, Clone, Default)]
#[diff(attr(
    #[derive(Debug, Serialize, Deserialize)]
))]
pub struct Image {
    pub filename: String,
    pub caption: Option<String>,
}

impl DiffPrint<Image> for ImageDiff {
    fn diff_print(&self, old: &Image, new: &Image, indent: usize, _name: &str) {
        self.filename
            .diff_print(&old.filename, &new.filename, indent, "filename");
        self.caption
            .diff_print(&old.caption, &new.caption, indent, "caption");
    }
}

impl DiffPrint<Vec<Image>> for diff::VecDiff<Image> {
    fn diff_print(&self, old: &Vec<Image>, new: &Vec<Image>, indent: usize, name: &str) {
        let indent_str = " ".repeat(indent);
        if self.0.is_empty() {
            return;
        }

        println!("{indent_str}{name}:");
        for diff in &self.0 {
            match diff {
                diff::VecDiffType::Inserted { index, .. } => {
                    println!(
                        "{indent_str}  +{}",
                        new.get(*index).cloned().unwrap_or_default().filename
                    );
                }
                diff::VecDiffType::Removed { index, .. } => {
                    println!(
                        "{indent_str}  -{}",
                        old.get(*index).cloned().unwrap_or_default().filename
                    );
                }
                diff::VecDiffType::Altered { index, changes } => {
                    let (o, n) = if old.get(*index).is_none() || new.get(*index).is_none() {
                        (Image::default(), Image::default())
                    } else {
                        (old[*index].clone(), new[*index].clone())
                    };

                    println!("{indent_str}  *");
                    for diff in changes {
                        diff.diff_print(&o, &n, indent + 4, "");
                    }
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff, Clone, Default)]
#[diff(attr(
    #[derive(Debug, Serialize, Deserialize)]
))]
pub struct Property {
    #[serde(flatten)]
    common: NamedCommon,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub alt_name: String,

    #[serde(rename = "override")]
    pub override_: bool,

    #[serde(rename = "type")]
    pub type_: Type,

    pub optional: bool,
    pub default: Option<PropertyDefault>,
}

impl Deref for Property {
    type Target = NamedCommon;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl Deref for PropertyDiff {
    type Target = NamedCommonDiff;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DiffPrint<Property> for PropertyDiff {
    fn diff_print(&self, old: &Property, new: &Property, indent: usize, _name: &str) {
        self.common.diff_print(&old.common, &new.common, indent, "");
        self.alt_name
            .diff_print(&old.alt_name, &new.alt_name, indent, "alt_name");
        self.override_
            .diff_print(&old.override_, &new.override_, indent, "override");
        self.type_
            .diff_print(&old.type_, &new.type_, indent, "type");
        self.optional
            .diff_print(&old.optional, &new.optional, indent, "optional");
        self.default
            .diff_print(&old.default, &new.default, indent, "default");
    }
}

impl DiffPrint<Vec<Property>> for diff::VecDiff<Property> {
    fn diff_print(&self, old: &Vec<Property>, new: &Vec<Property>, indent: usize, name: &str) {
        let indent_str = " ".repeat(indent);
        if self.0.is_empty() {
            return;
        }

        println!("{indent_str}{name}:");
        for diff in &self.0 {
            match diff {
                diff::VecDiffType::Inserted { index, .. } => {
                    println!(
                        "{indent_str}  +{}",
                        new.get(*index).cloned().unwrap_or_default().name
                    );
                }
                diff::VecDiffType::Removed { index, .. } => {
                    println!(
                        "{indent_str}  -{}",
                        old.get(*index).cloned().unwrap_or_default().name
                    );
                }
                diff::VecDiffType::Altered { index, changes } => {
                    let (o, n) = if old.get(*index).is_none() || new.get(*index).is_none() {
                        (Property::default(), Property::default())
                    } else {
                        (old[*index].clone(), new[*index].clone())
                    };

                    println!("{indent_str}  *{}", o.name);
                    for diff in changes {
                        diff.diff_print(&o, &n, indent + 4, "");
                    }
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff, Clone)]
#[diff(attr(
    #[derive(Debug, Serialize, Deserialize)]
))]
#[serde(untagged)]
pub enum PropertyDefault {
    String(String),
    Literal(Literal),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff)]
#[diff(attr(
    #[derive(Debug, Serialize, Deserialize)]
))]
pub struct CustomProperties {
    #[serde(flatten)]
    common: Common,

    pub key_type: Type,
    pub value_type: Type,
}

impl Deref for CustomProperties {
    type Target = Common;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl Deref for CustomPropertiesDiff {
    type Target = CommonDiff;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DiffPrint<CustomProperties> for CustomPropertiesDiff {
    fn diff_print(
        &self,
        old: &CustomProperties,
        new: &CustomProperties,
        indent: usize,
        _name: &str,
    ) {
        self.common.diff_print(&old.common, &new.common, indent, "");
        self.key_type
            .diff_print(&old.key_type, &new.key_type, indent, "key_type");
        self.value_type
            .diff_print(&old.value_type, &new.value_type, indent, "value_type");
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff, Clone)]
#[diff(attr(
    #[derive(Debug, Serialize, Deserialize)]
))]
#[serde(untagged)]
pub enum Type {
    Simple(String),
    Complex(Box<ComplexType>),
}

impl Type {
    #[must_use]
    pub fn as_simple(&self) -> Option<String> {
        match self {
            Self::Simple(s) => Some(s.clone()),
            Self::Complex(_) => None,
        }
    }

    #[must_use]
    pub fn as_complex(&self) -> Option<Box<ComplexType>> {
        match self {
            Self::Complex(c) => Some(c.clone()),
            Self::Simple(_) => None,
        }
    }
}

impl Default for Type {
    fn default() -> Self {
        Self::Simple(String::new())
    }
}

impl DiffPrint<Type> for TypeDiff {
    fn diff_print(&self, old: &Type, new: &Type, indent: usize, name: &str) {
        match self {
            Self::Simple(s) => s.diff_print(
                &old.as_simple().unwrap_or_default(),
                &new.as_simple().unwrap_or_default(),
                indent,
                name,
            ),
            Self::Complex(c) => c.diff_print(
                &old.as_complex().unwrap_or_default(),
                &new.as_complex().unwrap_or_default(),
                indent,
                name,
            ),
            Self::NoChange => {}
        }
    }
}

impl DiffPrint<Vec<Type>> for diff::VecDiff<Type> {
    fn diff_print(&self, old: &Vec<Type>, new: &Vec<Type>, indent: usize, name: &str) {
        let indent_str = " ".repeat(indent);
        if self.0.is_empty() {
            return;
        }

        println!("{indent_str}{name}:");
        for diff in &self.0 {
            match diff {
                diff::VecDiffType::Inserted { index, .. } => {
                    println!("{indent_str}  +{:?}", new[*index]);
                }
                diff::VecDiffType::Removed { index, .. } => {
                    println!("{indent_str}  -{:?}", old[*index]);
                }
                diff::VecDiffType::Altered { index, changes } => {
                    println!("{indent_str}  *{index}");
                    for diff in changes {
                        diff.diff_print(&old[*index], &new[*index], indent + 4, "");
                    }
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff, Clone)]
#[diff(attr(
    #[derive(Debug, Serialize, Deserialize)]
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

impl ComplexType {
    #[must_use]
    pub fn as_array(&self) -> Option<Type> {
        match self {
            Self::Array { value } => Some(value.clone()),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_dictionary(&self) -> Option<Self> {
        match self {
            Self::Dictionary { .. } => Some(self.clone()),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_tuple(&self) -> Option<Vec<Type>> {
        match self {
            Self::Tuple { values } => Some(values.clone()),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_union(&self) -> Option<Self> {
        match self {
            Self::Union { .. } => Some(self.clone()),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_type(&self) -> Option<Self> {
        match self {
            Self::Type { .. } => Some(self.clone()),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_literal(&self) -> Option<Literal> {
        match self {
            Self::Literal(l) => Some(l.clone()),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_struct(&self) -> Option<()> {
        match self {
            Self::Struct => Some(()),
            _ => None,
        }
    }
}

impl Default for ComplexType {
    fn default() -> Self {
        Self::Struct {}
    }
}

impl DiffPrint<ComplexType> for ComplexTypeDiff {
    fn diff_print(&self, old: &ComplexType, new: &ComplexType, indent: usize, name: &str) {
        if old == new {
            return;
        }

        let indent_str = " ".repeat(indent);

        match self {
            Self::Array { value } => value.diff_print(
                &old.as_array().unwrap_or_default(),
                &new.as_array().unwrap_or_default(),
                indent,
                name,
            ),
            Self::Dictionary { key, value } => {
                let old = old.as_dictionary().unwrap_or_default();
                let new = new.as_dictionary().unwrap_or_default();

                if let (
                    ComplexType::Dictionary {
                        key: old_k,
                        value: old_v,
                    },
                    ComplexType::Dictionary {
                        key: new_k,
                        value: new_v,
                    },
                ) = (old, new)
                {
                    println!("{indent_str}{name}:");
                    key.diff_print(&old_k, &new_k, indent + 2, "key");
                    value.diff_print(&old_v, &new_v, indent + 2, "value");
                }
            }
            Self::Tuple { values } => values.diff_print(
                &old.as_tuple().unwrap_or_default(),
                &new.as_tuple().unwrap_or_default(),
                indent,
                name,
            ),
            Self::Union {
                options,
                full_format,
            } => {
                let old = old.as_union().unwrap_or_default();
                let new = new.as_union().unwrap_or_default();

                if let (
                    ComplexType::Union {
                        options: old_o,
                        full_format: old_f,
                    },
                    ComplexType::Union {
                        options: new_o,
                        full_format: new_f,
                    },
                ) = (old, new)
                {
                    println!("{indent_str}{name}:");
                    options.diff_print(&old_o, &new_o, indent + 2, "options");
                    full_format.diff_print(&old_f, &new_f, indent + 2, "full_format");
                }
            }
            Self::Type { value, description } => {
                let old = old.as_type().unwrap_or_default();
                let new = new.as_type().unwrap_or_default();

                if let (
                    ComplexType::Type {
                        value: old_v,
                        description: old_d,
                    },
                    ComplexType::Type {
                        value: new_v,
                        description: new_d,
                    },
                ) = (old, new)
                {
                    println!("{indent_str}{name}:");
                    value.diff_print(&old_v, &new_v, indent + 2, "value");
                    description.diff_print(&old_d, &new_d, indent + 2, "description");
                }
            }
            Self::Literal(l) => l.diff_print(
                &old.as_literal().unwrap_or_default(),
                &new.as_literal().unwrap_or_default(),
                indent,
                name,
            ),
            Self::Struct => println!("{indent_str}{name}: struct"),
            Self::NoChange => {}
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff, Default, Clone)]
#[diff(attr(
    #[derive(Debug, Serialize, Deserialize)]
))]
pub struct Literal {
    pub value: LiteralValue,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
}

impl DiffPrint<Literal> for LiteralDiff {
    fn diff_print(&self, old: &Literal, new: &Literal, indent: usize, name: &str) {
        if old == new {
            return;
        }

        let indent_str = " ".repeat(indent);
        println!("{indent_str}{name}:");

        self.value
            .diff_print(&old.value, &new.value, indent, "value");
        self.description
            .diff_print(&old.description, &new.description, indent, "description");
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff, Clone)]
#[diff(attr(
    #[derive(Debug, Serialize, Deserialize)]
))]
#[serde(untagged)]
pub enum LiteralValue {
    String(String),
    UInt(u64),
    Int(i64),
    Float(f64),
    Boolean(bool),
}

impl LiteralValue {
    #[must_use]
    pub fn as_string(&self) -> Option<String> {
        match self {
            Self::String(s) => Some(s.clone()),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_uint(&self) -> Option<u64> {
        match self {
            Self::UInt(u) => Some(*u),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_int(&self) -> Option<i64> {
        match self {
            Self::Int(i) => Some(*i),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(f) => Some(*f),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(b) => Some(*b),
            _ => None,
        }
    }
}

impl Default for LiteralValue {
    fn default() -> Self {
        Self::String(String::new())
    }
}

impl DiffPrint<LiteralValue> for LiteralValueDiff {
    fn diff_print(&self, old: &LiteralValue, new: &LiteralValue, indent: usize, name: &str) {
        match self {
            Self::String(s) => s.diff_print(
                &old.as_string().unwrap_or_default(),
                &new.as_string().unwrap_or_default(),
                indent,
                name,
            ),
            Self::UInt(u) => u.diff_print(
                &old.as_uint().unwrap_or_default(),
                &new.as_uint().unwrap_or_default(),
                indent,
                name,
            ),
            Self::Int(i) => i.diff_print(
                &old.as_int().unwrap_or_default(),
                &new.as_int().unwrap_or_default(),
                indent,
                name,
            ),
            Self::Float(f) => f.diff_print(
                &old.as_float().unwrap_or_default(),
                &new.as_float().unwrap_or_default(),
                indent,
                name,
            ),
            Self::Boolean(b) => b.diff_print(
                &old.as_boolean().unwrap_or_default(),
                &new.as_boolean().unwrap_or_default(),
                indent,
                name,
            ),
            Self::NoChange => {}
        }
    }
}
