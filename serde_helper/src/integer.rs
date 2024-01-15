#![allow(clippy::module_name_repetitions)]

pub use i8::*;
mod i8 {
    #[must_use]
    pub const fn i8_1() -> i8 {
        1
    }

    #[must_use]
    pub const fn i8_100() -> i8 {
        100
    }

    #[must_use]
    pub const fn is_1_i8(value: &i8) -> bool {
        *value == i8_1()
    }

    #[must_use]
    pub const fn is_100_i8(value: &i8) -> bool {
        *value == i8_100()
    }
}

pub use u8::*;
mod u8 {
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
}

pub use u16::*;
mod u16 {
    #[must_use]
    pub const fn u16_1() -> u16 {
        1
    }

    #[must_use]
    pub const fn is_1_u16(value: &u16) -> bool {
        *value == u16_1()
    }
}

pub use u32::*;
mod u32 {
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
    pub const fn u32_30() -> u32 {
        30
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
    pub const fn is_30_u32(value: &u32) -> bool {
        *value == u32_30()
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
}
