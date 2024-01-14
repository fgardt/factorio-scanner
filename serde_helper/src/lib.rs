#![forbid(unsafe_code)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]

use num::{Bounded, FromPrimitive, Integer, ToPrimitive};
use serde::Deserializer;

#[allow(clippy::missing_errors_doc)]
pub fn truncating_deserializer<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Bounded + Integer + ToPrimitive + FromPrimitive,
{
    use serde::{de::Error as deError, Deserialize};

    let tmp = f64::deserialize(deserializer)?;

    let Some(min) = T::min_value().to_f64() else {
        return Err(deError::custom(format!(
            "unable to convert {} min value to f64",
            std::any::type_name::<T>()
        )));
    };
    let Some(max) = T::max_value().to_f64() else {
        return Err(deError::custom(format!(
            "unable to convert {} max value to f64",
            std::any::type_name::<T>()
        )));
    };

    if tmp < min || tmp > max {
        return Err(deError::custom(format!(
            "{} out of range: {}",
            std::any::type_name::<T>(),
            tmp,
        )));
    }

    T::from_f64(tmp).map_or_else(
        || {
            Err(deError::custom(format!(
                "unable to truncate {} to {}",
                tmp,
                std::any::type_name::<T>(),
            )))
        },
        |res| Ok(res),
    )
}

#[allow(clippy::missing_errors_doc)]
pub fn truncating_opt_deserializer<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Bounded + Integer + ToPrimitive + FromPrimitive,
{
    use serde::{de::Error as deError, Deserialize};

    let Some(tmp) = Option::<f64>::deserialize(deserializer)? else {
        return Ok(None);
    };

    let Some(min) = T::min_value().to_f64() else {
        return Err(deError::custom(format!(
            "unable to convert {} min value to f64",
            std::any::type_name::<T>()
        )));
    };
    let Some(max) = T::max_value().to_f64() else {
        return Err(deError::custom(format!(
            "unable to convert {} max value to f64",
            std::any::type_name::<T>()
        )));
    };

    if tmp < min || tmp > max {
        return Err(deError::custom(format!(
            "{} out of range: {}",
            std::any::type_name::<T>(),
            tmp,
        )));
    }

    T::from_f64(tmp).map_or_else(
        || {
            Err(deError::custom(format!(
                "unable to truncate {} to {}",
                tmp,
                std::any::type_name::<T>(),
            )))
        },
        |res| Ok(Some(res)),
    )
}

#[must_use]
pub const fn bool_true() -> bool {
    true
}

#[must_use]
pub const fn i8_1() -> i8 {
    1
}

#[must_use]
pub const fn i8_100() -> i8 {
    100
}

#[must_use]
pub const fn u8_1() -> u8 {
    1
}

#[must_use]
pub const fn u8_2() -> u8 {
    2
}

#[must_use]
pub const fn u8_3() -> u8 {
    3
}

#[must_use]
pub const fn u8_4() -> u8 {
    4
}

#[must_use]
pub const fn u8_5() -> u8 {
    5
}

#[must_use]
pub const fn u8_6() -> u8 {
    6
}

#[must_use]
pub const fn u8_7() -> u8 {
    7
}

#[must_use]
pub const fn u8_8() -> u8 {
    8
}

#[must_use]
pub const fn u8_9() -> u8 {
    9
}

#[must_use]
pub const fn u8_10() -> u8 {
    10
}

#[must_use]
pub const fn u8_11() -> u8 {
    11
}

#[must_use]
pub const fn u8_12() -> u8 {
    12
}

#[must_use]
pub const fn u8_13() -> u8 {
    13
}

#[must_use]
pub const fn u8_14() -> u8 {
    14
}

#[must_use]
pub const fn u8_15() -> u8 {
    15
}

#[must_use]
pub const fn u8_16() -> u8 {
    16
}

#[must_use]
pub const fn u8_17() -> u8 {
    17
}

#[must_use]
pub const fn u8_18() -> u8 {
    18
}

#[must_use]
pub const fn u8_19() -> u8 {
    19
}

#[must_use]
pub const fn u8_20() -> u8 {
    20
}

#[must_use]
pub const fn u8_30() -> u8 {
    30
}

#[must_use]
pub const fn u8_50() -> u8 {
    50
}

#[must_use]
pub const fn u8_120() -> u8 {
    120
}

#[must_use]
pub const fn u8_max() -> u8 {
    u8::MAX
}

#[must_use]
pub const fn u16_1() -> u16 {
    1
}

#[must_use]
pub const fn u32_1() -> u32 {
    1
}

#[must_use]
pub const fn u32_4() -> u32 {
    4
}

#[must_use]
pub const fn u32_8() -> u32 {
    8
}

#[must_use]
pub const fn u32_10() -> u32 {
    10
}

#[must_use]
pub const fn u32_60() -> u32 {
    60
}

#[must_use]
pub const fn u32_120() -> u32 {
    120
}

#[must_use]
pub const fn u32_3600() -> u32 {
    3600
}

#[must_use]
pub const fn f32_001() -> f32 {
    0.01
}

#[must_use]
pub const fn f32_1() -> f32 {
    1.0
}

#[must_use]
pub const fn f64_001() -> f64 {
    0.01
}

#[must_use]
pub const fn f64_1_64() -> f64 {
    const RES: f64 = 1.0 / 64.0;
    RES
}

#[must_use]
pub const fn f64_1_60() -> f64 {
    const RES: f64 = 1.0 / 60.0;
    RES
}

#[must_use]
pub const fn f64_2_32() -> f64 {
    const RES: f64 = 2.0 / 32.0;
    RES
}

#[must_use]
pub const fn f64_02() -> f64 {
    0.2
}

#[must_use]
pub const fn f64_quarter() -> f64 {
    0.25
}

#[must_use]
pub const fn f64_03() -> f64 {
    0.3
}

#[must_use]
pub const fn f64_half() -> f64 {
    0.5
}

#[must_use]
pub const fn f64_075() -> f64 {
    0.75
}

#[must_use]
pub const fn f64_095() -> f64 {
    0.95
}

#[must_use]
pub const fn f64_1() -> f64 {
    1.0
}

#[must_use]
pub const fn f64_1_5() -> f64 {
    1.5
}

#[must_use]
pub const fn f64_3() -> f64 {
    3.0
}

#[must_use]
pub const fn f64_10() -> f64 {
    10.0
}

#[must_use]
pub const fn f64_15() -> f64 {
    15.0
}

#[must_use]
pub const fn f64_1000() -> f64 {
    1000.0
}

#[must_use]
pub const fn f64_max() -> f64 {
    f64::MAX
}

#[must_use]
pub const fn is_0_i8(value: &i8) -> bool {
    *value == 0
}

#[must_use]
pub const fn is_1_i8(value: &i8) -> bool {
    *value == i8_1()
}

#[must_use]
pub const fn is_100_i8(value: &i8) -> bool {
    *value == i8_100()
}

#[must_use]
pub const fn is_0_i16(value: &i16) -> bool {
    *value == 0
}

#[must_use]
pub const fn is_0_u8(value: &u8) -> bool {
    *value == 0
}

#[must_use]
pub const fn is_1_u8(value: &u8) -> bool {
    *value == u8_1()
}

#[must_use]
pub const fn is_2_u8(value: &u8) -> bool {
    *value == u8_2()
}

#[must_use]
pub const fn is_3_u8(value: &u8) -> bool {
    *value == u8_3()
}

#[must_use]
pub const fn is_4_u8(value: &u8) -> bool {
    *value == u8_4()
}

#[must_use]
pub const fn is_5_u8(value: &u8) -> bool {
    *value == u8_5()
}

#[must_use]
pub const fn is_6_u8(value: &u8) -> bool {
    *value == u8_6()
}

#[must_use]
pub const fn is_7_u8(value: &u8) -> bool {
    *value == u8_7()
}

#[must_use]
pub const fn is_8_u8(value: &u8) -> bool {
    *value == u8_8()
}

#[must_use]
pub const fn is_9_u8(value: &u8) -> bool {
    *value == u8_9()
}

#[must_use]
pub const fn is_10_u8(value: &u8) -> bool {
    *value == u8_10()
}

#[must_use]
pub const fn is_11_u8(value: &u8) -> bool {
    *value == u8_11()
}

#[must_use]
pub const fn is_12_u8(value: &u8) -> bool {
    *value == u8_12()
}

#[must_use]
pub const fn is_13_u8(value: &u8) -> bool {
    *value == u8_13()
}

#[must_use]
pub const fn is_14_u8(value: &u8) -> bool {
    *value == u8_14()
}

#[must_use]
pub const fn is_15_u8(value: &u8) -> bool {
    *value == u8_15()
}

#[must_use]
pub const fn is_16_u8(value: &u8) -> bool {
    *value == u8_16()
}

#[must_use]
pub const fn is_17_u8(value: &u8) -> bool {
    *value == u8_17()
}

#[must_use]
pub const fn is_18_u8(value: &u8) -> bool {
    *value == u8_18()
}

#[must_use]
pub const fn is_19_u8(value: &u8) -> bool {
    *value == u8_19()
}

#[must_use]
pub const fn is_20_u8(value: &u8) -> bool {
    *value == u8_20()
}

#[must_use]
pub const fn is_30_u8(value: &u8) -> bool {
    *value == u8_30()
}

#[must_use]
pub const fn is_50_u8(value: &u8) -> bool {
    *value == u8_50()
}

#[must_use]
pub const fn is_120_u8(value: &u8) -> bool {
    *value == u8_120()
}

#[must_use]
pub const fn is_max_u8(value: &u8) -> bool {
    *value == u8_max()
}

#[must_use]
pub const fn is_0_u16(value: &u16) -> bool {
    *value == 0
}

#[must_use]
pub const fn is_1_u16(value: &u16) -> bool {
    *value == u16_1()
}

#[must_use]
pub const fn is_0_i32(value: &i32) -> bool {
    *value == 0
}

#[must_use]
pub const fn is_0_u32(value: &u32) -> bool {
    *value == 0
}

#[must_use]
pub const fn is_1_u32(value: &u32) -> bool {
    *value == u32_1()
}

#[must_use]
pub const fn is_4_u32(value: &u32) -> bool {
    *value == u32_4()
}

#[must_use]
pub const fn is_8_u32(value: &u32) -> bool {
    *value == u32_8()
}

#[must_use]
pub const fn is_10_u32(value: &u32) -> bool {
    *value == u32_10()
}

#[must_use]
pub const fn is_60_u32(value: &u32) -> bool {
    *value == u32_60()
}

#[must_use]
pub const fn is_120_u32(value: &u32) -> bool {
    *value == u32_120()
}

#[must_use]
pub const fn is_3600_u32(value: &u32) -> bool {
    *value == u32_3600()
}

#[must_use]
pub const fn is_0_u64(value: &u64) -> bool {
    *value == 0
}

#[must_use]
pub fn is_0_f32(value: &f32) -> bool {
    value.abs() < f32::EPSILON
}

#[must_use]
pub fn is_001_f32(value: &f32) -> bool {
    (*value - f32_001()).abs() < f32::EPSILON
}

#[must_use]
pub fn is_1_f32(value: &f32) -> bool {
    (*value - f32_1()).abs() < f32::EPSILON
}

#[must_use]
pub fn is_0_f64(value: &f64) -> bool {
    *value == 0.0
}

#[must_use]
pub fn is_001_f64(value: &f64) -> bool {
    (*value - f64_001()).abs() < f64::EPSILON
}

#[must_use]
pub fn is_1_64_f64(value: &f64) -> bool {
    (*value - f64_1_64()).abs() < f64::EPSILON
}

#[must_use]
pub fn is_1_60_f64(value: &f64) -> bool {
    (*value - f64_1_60()).abs() < f64::EPSILON
}

#[must_use]
pub fn is_2_32_f64(value: &f64) -> bool {
    (*value - f64_2_32()).abs() < f64::EPSILON
}

#[must_use]
pub fn is_02_f64(value: &f64) -> bool {
    (*value - f64_02()).abs() < f64::EPSILON
}

#[must_use]
pub fn is_03_f64(value: &f64) -> bool {
    (*value - f64_03()).abs() < f64::EPSILON
}

#[must_use]
pub fn is_quarter_f64(value: &f64) -> bool {
    (*value - f64_quarter()).abs() < f64::EPSILON
}

#[must_use]
pub fn is_half_f64(value: &f64) -> bool {
    (*value - f64_half()).abs() < f64::EPSILON
}

#[must_use]
pub fn is_075_f64(value: &f64) -> bool {
    (*value - f64_075()).abs() < f64::EPSILON
}

#[must_use]
pub fn is_095_f64(value: &f64) -> bool {
    (*value - f64_095()).abs() < f64::EPSILON
}

#[must_use]
pub fn is_1_f64(value: &f64) -> bool {
    (*value - f64_1()).abs() < f64::EPSILON
}

#[must_use]
pub fn is_1_5_f64(value: &f64) -> bool {
    (*value - f64_1_5()).abs() < f64::EPSILON
}

#[must_use]
pub fn is_3_f64(value: &f64) -> bool {
    (*value - f64_3()).abs() < f64::EPSILON
}

#[must_use]
pub fn is_10_f64(value: &f64) -> bool {
    (*value - f64_10()).abs() < f64::EPSILON
}

#[must_use]
pub fn is_15_f64(value: &f64) -> bool {
    (*value - f64_15()).abs() < f64::EPSILON
}

#[must_use]
pub fn is_1000_f64(value: &f64) -> bool {
    (*value - f64_1000()).abs() < f64::EPSILON
}

#[must_use]
pub fn is_max_f64(value: &f64) -> bool {
    (*value - f64_max()).abs() < f64::EPSILON
}
