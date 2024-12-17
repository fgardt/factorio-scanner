#![allow(clippy::module_name_repetitions)]

macro_rules! float_default {
    ($kind:ty, $(($id:ident, $val:expr)),+) => {
        paste::paste! {
            pub use $kind::*;
            mod $kind {
                $(
                #[must_use]
                pub const fn [< $kind $id >]() -> $kind {
                    $val
                }

                #[must_use]
                pub fn [< is $id _ $kind >](value: &$kind) -> bool {
                    (*value - [< $kind $id >]()).abs() < $kind::EPSILON
                }
                )+
            }
        }
    };
}

float_default!(
    f32,
    (_n1, -1.0),
    (_001, 0.01),
    (_005, 0.05),
    (_01, 0.1),
    (_03, 0.3),
    (_05, 0.5),
    (_087, 0.87),
    (_09, 0.9),
    (_1, 1.0),
    (_3, 3.0),
    (_min, f32::MIN),
    (_max, f32::MAX)
);

float_default!(
    f64,
    (_001, 0.01),
    (_1_64, 1.0 / 64.0),
    (_1_60, 1.0 / 60.0),
    (_2_32, 2.0 / 32.0),
    (_02, 0.2),
    (_025, 0.25),
    (_05, 0.5),
    (_07, 0.7),
    (_075, 0.75),
    (_095, 0.95),
    (_1, 1.0),
    (_1_333, 1.333),
    (_1_5, 1.5),
    (_3, 3.0),
    (_5, 5.0),
    (_10, 10.0),
    (_15, 15.0),
    (_1000, 1000.0),
    (_min, f64::MIN),
    (_max, f64::MAX)
);
