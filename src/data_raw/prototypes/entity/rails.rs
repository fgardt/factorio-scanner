use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::EntityWithOwnerPrototype;
use crate::data_raw::types::*;

/// [`Prototypes/RailPrototype`](https://lua-api.factorio.com/latest/prototypes/RailPrototype.html)
pub type RailPrototype<T> = EntityWithOwnerPrototype<RailData<T>>;

/// [`Prototypes/RailPrototype`](https://lua-api.factorio.com/latest/prototypes/RailPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RailData<T> {
    pub pictures: RailPictureSet,

    #[serde(flatten)]
    pub child: T,
    // not implemented
    // pub walking_sound: Option<Sound>,
}

/// [`Prototypes/CurvedRailPrototype`](https://lua-api.factorio.com/latest/prototypes/CurvedRailPrototype.html)
pub type CurvedRailPrototype = RailPrototype<CurvedRailData>;

/// [`Prototypes/CurvedRailPrototype`](https://lua-api.factorio.com/latest/prototypes/CurvedRailPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct CurvedRailData {
    pub bending_type: Option<CurvedBendType>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CurvedBendType {
    Turn,
}

/// [`Prototypes/StraightRailPrototype`](https://lua-api.factorio.com/latest/prototypes/StraightRailPrototype.html)
pub type StraightRailPrototype = RailPrototype<StraightRailData>;

/// [`Prototypes/StraightRailPrototype`](https://lua-api.factorio.com/latest/prototypes/StraightRailPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct StraightRailData {
    pub bending_type: Option<StraightBendType>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StraightBendType {
    Straight,
}
