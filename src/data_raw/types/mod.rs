#![allow(clippy::struct_excessive_bools)]

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

pub mod graphics;

#[allow(clippy::wildcard_imports)]
pub use graphics::*;

// TODO: support the array specification

///[`Types/Color`](https://lua-api.factorio.com/latest/types/Color.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct Color {
    #[serde(default, skip_serializing_if = "helper::is_zero_f64")]
    r: f64,
    #[serde(default, skip_serializing_if = "helper::is_zero_f64")]
    g: f64,
    #[serde(default, skip_serializing_if = "helper::is_zero_f64")]
    b: f64,
    #[serde(
        default = "helper::one_f64",
        skip_serializing_if = "helper::is_one_f64"
    )]
    a: f64,
}

impl Color {
    const fn white() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }
    fn is_white(color: &Self) -> bool {
        color.r == 1.0 && color.g == 1.0 && color.b == 1.0 && color.a == 1.0
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}

/// [`Types/Vector`](https://lua-api.factorio.com/latest/types/Vector.html)
pub type Vector = (f64, f64);

/// [`Types/FileName`](https://lua-api.factorio.com/latest/types/FileName.html)
pub type FileName = String;

/// [`Types/Order`](https://lua-api.factorio.com/latest/types/Order.html)
pub type Order = String;

///[`Types/RealOrientation`](https://lua-api.factorio.com/latest/types/RealOrientation.html)
pub type RealOrientation = f64;

/// [`Types/LocalisedString`](https://lua-api.factorio.com/latest/types/LocalisedString.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LocalisedString {
    Bool(bool),
    String(String),
    Array(Vec<LocalisedString>),
}

/// [`Types/RadiusVisualisationSpecification`](https://lua-api.factorio.com/latest/types/RadiusVisualisationSpecification.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RadiusVisualisationSpecification {
    pub sprite: Option<Sprite>,

    #[serde(default, skip_serializing_if = "helper::is_zero_f64")]
    pub distance: f64,

    pub offset: Option<Vector>,

    #[serde(default = "helper::true_bool", skip_serializing_if = "Clone::clone")]
    pub draw_in_cursor: bool,

    #[serde(default = "helper::true_bool", skip_serializing_if = "Clone::clone")]
    pub draw_on_selection: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LightDefinitionType {
    #[default]
    Basic,
    Oriented,
}

/// [`Types/LightDefinition`](https://lua-api.factorio.com/latest/types/LightDefinition.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct LightDefinition {
    // TODO: skip serializing if is default
    #[serde(default, rename = "type")]
    pub type_: LightDefinitionType,

    pub picture: Option<Sprite>,

    #[serde(default, skip_serializing_if = "helper::is_zero_f64")]
    pub rotation_shift: RealOrientation,

    pub intensity: f64,
    pub size: f64,

    #[serde(default, skip_serializing_if = "helper::is_zero_f64")]
    pub source_orientation_offset: RealOrientation,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub add_perspective: bool,

    pub shift: Option<Vector>,
    pub color: Option<Color>,

    #[serde(default, skip_serializing_if = "helper::is_zero_f64")]
    pub minimum_darkness: f64,
}

/// [`Types/CircuitConnectorSprites`](https://lua-api.factorio.com/latest/types/CircuitConnectorSprites.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitConnectorSprites {
    pub led_red: Sprite,
    pub led_green: Sprite,
    pub led_blue: Sprite,
    pub led_light: LightDefinition,

    pub connector_main: Option<Sprite>,
    pub connector_shadow: Option<Sprite>,

    pub wire_pins: Option<Sprite>,
    pub wire_pins_shadow: Option<Sprite>,

    pub led_blue_off: Option<Sprite>,
    pub led_blue_light_offset: Option<Vector>,
    pub red_green_led_light_offset: Option<Vector>,
}

/// [`Types/BoxSpecification`](https://lua-api.factorio.com/latest/types/BoxSpecification.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct BoxSpecification {
    pub sprite: Sprite,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_whole_box: bool,

    pub side_length: Option<f64>,

    pub side_height: Option<f64>,

    pub max_side_length: Option<f64>,
}

/// [`Types/BeamAnimationSet`](https://lua-api.factorio.com/latest/types/BeamAnimationSet.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct BeamAnimationSet {
    pub start: Option<Animation>,
    pub ending: Option<Animation>,
    pub head: Option<Animation>,
    pub tail: Option<Animation>,
    pub body: Option<AnimationVariations>,
}

/// [`Types/PumpConnectorGraphicsAnimation`](https://lua-api.factorio.com/latest/types/PumpConnectorGraphicsAnimation.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct PumpConnectorGraphicsAnimation {
    pub standup_base: Option<Animation>,
    pub standup_top: Option<Animation>,
    pub standup_shadow: Option<Animation>,
    pub connector: Option<Animation>,
    pub connector_shadow: Option<Animation>,
}

/// [`Types/PumpConnectorGraphics`](https://lua-api.factorio.com/latest/types/PumpConnectorGraphics.html)
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Serialize, Deserialize)]
pub struct PumpConnectorGraphics {
    pub north: Vec<PumpConnectorGraphicsAnimation>,
    pub east: Vec<PumpConnectorGraphicsAnimation>,
    pub south: Vec<PumpConnectorGraphicsAnimation>,
    pub west: Vec<PumpConnectorGraphicsAnimation>,
}

/// [`Types/CharacterArmorAnimation`](https://lua-api.factorio.com/latest/types/CharacterArmorAnimation.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterArmorAnimation {
    pub idle: RotatedAnimation,
    pub idle_with_gun: RotatedAnimation,
    pub running: RotatedAnimation,
    pub running_with_gun: RotatedAnimation,
    pub mining_with_tool: RotatedAnimation,
    pub flipped_shadow_running_with_gun: Option<RotatedAnimation>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub armors: Vec<String>,
}

/// [`Types/TransportBeltAnimationSet`](https://lua-api.factorio.com/latest/types/TransportBeltAnimationSet.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct TransportBeltAnimationSet {
    pub animation_set: RotatedAnimation,

    #[serde(default = "helper::one_u8", skip_serializing_if = "helper::is_one_u8")]
    pub east_index: u8,
    #[serde(default = "helper::u8_2", skip_serializing_if = "helper::is_2_u8")]
    pub west_index: u8,
    #[serde(default = "helper::u8_3", skip_serializing_if = "helper::is_3_u8")]
    pub north_index: u8,
    #[serde(default = "helper::u8_4", skip_serializing_if = "helper::is_4_u8")]
    pub south_index: u8,

    #[serde(default = "helper::u8_13", skip_serializing_if = "helper::is_13_u8")]
    pub starting_south_index: u8,
    #[serde(default = "helper::u8_14", skip_serializing_if = "helper::is_14_u8")]
    pub ending_south_index: u8,
    #[serde(default = "helper::u8_15", skip_serializing_if = "helper::is_15_u8")]
    pub starting_west_index: u8,
    #[serde(default = "helper::u8_16", skip_serializing_if = "helper::is_16_u8")]
    pub ending_west_index: u8,
    #[serde(default = "helper::u8_17", skip_serializing_if = "helper::is_17_u8")]
    pub starting_north_index: u8,
    #[serde(default = "helper::u8_18", skip_serializing_if = "helper::is_18_u8")]
    pub ending_north_index: u8,
    #[serde(default = "helper::u8_19", skip_serializing_if = "helper::is_19_u8")]
    pub starting_east_index: u8,
    #[serde(default = "helper::u8_20", skip_serializing_if = "helper::is_20_u8")]
    pub ending_east_index: u8,

    pub ending_patch: Option<Sprite4Way>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub ends_with_stopper: bool,
}

/// [`Types/TransportBeltAnimationSetWithCorners`](https://lua-api.factorio.com/latest/types/TransportBeltAnimationSetWithCorners.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct TransportBeltAnimationSetWithCorners {
    #[serde(default = "helper::u8_5", skip_serializing_if = "helper::is_5_u8")]
    pub east_to_north_index: u8,
    #[serde(default = "helper::u8_6", skip_serializing_if = "helper::is_6_u8")]
    pub north_to_east_index: u8,
    #[serde(default = "helper::u8_7", skip_serializing_if = "helper::is_7_u8")]
    pub west_to_north_index: u8,
    #[serde(default = "helper::u8_8", skip_serializing_if = "helper::is_8_u8")]
    pub north_to_west_index: u8,
    #[serde(default = "helper::u8_9", skip_serializing_if = "helper::is_9_u8")]
    pub south_to_east_index: u8,
    #[serde(default = "helper::u8_10", skip_serializing_if = "helper::is_10_u8")]
    pub east_to_south_index: u8,
    #[serde(default = "helper::u8_11", skip_serializing_if = "helper::is_11_u8")]
    pub south_to_west_index: u8,
    #[serde(default = "helper::u8_12", skip_serializing_if = "helper::is_12_u8")]
    pub west_to_south_index: u8,

    #[serde(flatten)]
    pub animation_set: TransportBeltAnimationSet,
}

mod helper {
    pub const fn true_bool() -> bool {
        true
    }

    pub const fn half_f64() -> f64 {
        0.5
    }

    pub const fn one_f64() -> f64 {
        1.0
    }

    pub const fn one_u8() -> u8 {
        1
    }

    pub const fn u8_2() -> u8 {
        2
    }

    pub const fn u8_3() -> u8 {
        3
    }

    pub const fn u8_4() -> u8 {
        4
    }

    pub const fn u8_5() -> u8 {
        5
    }

    pub const fn u8_6() -> u8 {
        6
    }

    pub const fn u8_7() -> u8 {
        7
    }

    pub const fn u8_8() -> u8 {
        8
    }

    pub const fn u8_9() -> u8 {
        9
    }

    pub const fn u8_10() -> u8 {
        10
    }

    pub const fn u8_11() -> u8 {
        11
    }

    pub const fn u8_12() -> u8 {
        12
    }

    pub const fn u8_13() -> u8 {
        13
    }

    pub const fn u8_14() -> u8 {
        14
    }

    pub const fn u8_15() -> u8 {
        15
    }

    pub const fn u8_16() -> u8 {
        16
    }

    pub const fn u8_17() -> u8 {
        17
    }

    pub const fn u8_18() -> u8 {
        18
    }

    pub const fn u8_19() -> u8 {
        19
    }

    pub const fn u8_20() -> u8 {
        20
    }

    pub const fn one_u32() -> u32 {
        1
    }

    pub const fn four_u32() -> u32 {
        4
    }

    pub const fn eight_u32() -> u32 {
        8
    }

    pub const fn max_f64() -> f64 {
        f64::MAX
    }

    pub fn is_zero_f64(value: &f64) -> bool {
        *value == 0.0
    }

    pub fn is_half_f64(value: &f64) -> bool {
        *value == 0.5
    }

    pub fn is_one_f64(value: &f64) -> bool {
        *value == 1.0
    }

    pub fn is_max_f64(value: &f64) -> bool {
        *value == f64::MAX
    }

    pub const fn is_zero_i16(value: &i16) -> bool {
        *value == 0
    }

    pub const fn is_zero_u8(value: &u8) -> bool {
        *value == 0
    }

    pub const fn is_one_u8(value: &u8) -> bool {
        *value == 1
    }

    pub const fn is_2_u8(value: &u8) -> bool {
        *value == 2
    }

    pub const fn is_3_u8(value: &u8) -> bool {
        *value == 3
    }

    pub const fn is_4_u8(value: &u8) -> bool {
        *value == 4
    }

    pub const fn is_5_u8(value: &u8) -> bool {
        *value == 5
    }

    pub const fn is_6_u8(value: &u8) -> bool {
        *value == 6
    }

    pub const fn is_7_u8(value: &u8) -> bool {
        *value == 7
    }

    pub const fn is_8_u8(value: &u8) -> bool {
        *value == 8
    }

    pub const fn is_9_u8(value: &u8) -> bool {
        *value == 9
    }

    pub const fn is_10_u8(value: &u8) -> bool {
        *value == 10
    }

    pub const fn is_11_u8(value: &u8) -> bool {
        *value == 11
    }

    pub const fn is_12_u8(value: &u8) -> bool {
        *value == 12
    }

    pub const fn is_13_u8(value: &u8) -> bool {
        *value == 13
    }

    pub const fn is_14_u8(value: &u8) -> bool {
        *value == 14
    }

    pub const fn is_15_u8(value: &u8) -> bool {
        *value == 15
    }

    pub const fn is_16_u8(value: &u8) -> bool {
        *value == 16
    }

    pub const fn is_17_u8(value: &u8) -> bool {
        *value == 17
    }

    pub const fn is_18_u8(value: &u8) -> bool {
        *value == 18
    }

    pub const fn is_19_u8(value: &u8) -> bool {
        *value == 19
    }

    pub const fn is_20_u8(value: &u8) -> bool {
        *value == 20
    }

    pub const fn is_zero_u32(value: &u32) -> bool {
        *value == 0
    }

    pub const fn is_one_u32(value: &u32) -> bool {
        *value == 1
    }

    pub const fn is_zero_u64(value: &u64) -> bool {
        *value == 0
    }
}
