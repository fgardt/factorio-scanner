use serde::{Deserialize, Serialize};
use serde_helper as helper;
use types::{Icon, RealOrientation, SpaceLocationID, StarMapIcon};

use crate::helper_macro::namespace_struct;

/// [`Prototypes/SpaceLocationPrototype`](https://lua-api.factorio.com/latest/prototypes/SpaceLocationPrototype.html)
pub type SpaceLocationPrototype = crate::BasePrototype<SpaceLocationPrototypeData>;

/// [`Prototypes/SpaceLocationPrototype`](https://lua-api.factorio.com/latest/prototypes/SpaceLocationPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct SpaceLocationPrototypeData {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub gravity_pull: f64,

    pub distance: f64,
    pub orientation: RealOrientation,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub magnitude: f64,

    pub parked_platforms_orientation: Option<RealOrientation>,

    #[serde(default = "default_label", skip_serializing_if = "is_default_label")]
    pub label_orientation: RealOrientation,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_orbit: bool,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub solar_power_in_space: f64,

    #[serde(default = "helper::f64_01", skip_serializing_if = "helper::is_01_f64")]
    pub asteroid_spawn_influence: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub fly_condition: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub auto_save_on_first_trip: bool,

    #[serde(flatten)]
    pub icon: Icon,

    #[serde(flatten)]
    pub starmap_icon: Option<StarMapIcon>,

    pub starmap_icon_orientation: Option<RealOrientation>,
    // not implemented
    // procession_graphic_catalogue
    // procession_audio_catalogue
    // platform_procession_set
    // planet_procession_set
    // asteroid_spawn_definitions
}

const fn default_label() -> RealOrientation {
    RealOrientation::new(0.25)
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_default_label(orientation: &RealOrientation) -> bool {
    default_label() == *orientation
}

namespace_struct! {
    AllTypes,
    SpaceLocationID,
    "space-location"
}
