use diff::Diff;
use serde::{Deserialize, Serialize};

pub mod prototype;
pub mod runtime;

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff)]
#[diff(attr(
    #[derive(Debug)]
))]
#[serde(rename_all = "lowercase")]
pub enum Application {
    Factorio,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff)]
#[diff(attr(
    #[derive(Debug)]
))]
#[serde(rename_all = "lowercase")]
pub enum Stage {
    Prototype,
    Runtime,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff)]
#[diff(attr(
    #[derive(Debug)]
))]
pub struct Common {
    pub application: Application,
    pub stage: Stage,
    pub application_version: String,
    pub api_version: u8,
}
