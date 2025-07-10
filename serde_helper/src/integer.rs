#![allow(clippy::module_name_repetitions)]

macro_rules! int_default {
    ($kind:ty, $($val:literal),+) => {
        paste::paste! {
            pub use $kind::*;
            mod $kind {
                $(
                #[must_use]
                pub const fn [< $kind _ $val >]() -> $kind {
                    $val
                }

                #[must_use]
                pub const fn [< is_ $val _ $kind >](value: &$kind) -> bool {
                    *value == [< $kind _ $val >]()
                }
                )+

                #[must_use]
                pub const fn [<$kind _min>]() -> $kind {
                    $kind::MIN
                }

                #[must_use]
                pub const fn [< is_min_ $kind >](value: &$kind) -> bool {
                    *value == [<$kind _min>]()
                }

                #[must_use]
                pub const fn [<$kind _max>]() -> $kind {
                    $kind::MAX
                }

                #[must_use]
                pub const fn [< is_max_ $kind >](value: &$kind) -> bool {
                    *value == [<$kind _max>]()
                }
            }
        }
    };
}

int_default!(i8, 1, 100);
int_default!(
    u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 30, 50, 55, 56, 100,
    120
);
int_default!(i16, 64);
int_default!(u16, 1, 5, 39);
int_default!(i32, 1);
int_default!(u32, 1, 3, 4, 6, 8, 10, 30, 60, 80, 120, 400, 3600);
