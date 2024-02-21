use serde::{Deserialize, Serialize};
use serde_helper as helper;

mod decon;
mod upgrade;

pub use decon::*;
pub use upgrade::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlannerData<T: Default + PartialEq> {
    #[serde(flatten, default, skip_serializing_if = "helper::is_default")]
    settings: T,
}

impl<T: Default + PartialEq> std::ops::Deref for PlannerData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.settings
    }
}
