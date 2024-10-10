use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::QualityCondition;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "kebab-case", deny_unknown_fields)]
pub enum ParameterData {
    Id {
        #[serde(rename = "not-parametrised")]
        not_parametrised: Option<bool>,
        name: Option<String>,
        id: String,

        #[serde(rename = "quality-condition")]
        quality_condition: Option<QualityCondition>,

        #[serde(rename = "ingredient-of")]
        ingredient_of: Option<String>,
    },
    Number {
        #[serde(rename = "not-parametrised")]
        not_parametrised: Option<bool>,
        number: String,
        name: Option<String>,
        variable: Option<String>,
        formula: Option<String>,
    },
}
