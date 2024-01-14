#![allow(clippy::module_name_repetitions)]

pub use f32::*;
mod f32 {
    #[must_use]
    pub const fn f32_001() -> f32 {
        0.01
    }

    #[must_use]
    pub const fn f32_1() -> f32 {
        1.0
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
}

pub use f64::*;
mod f64 {
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
}
