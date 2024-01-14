#![forbid(unsafe_code)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]

use num::{Bounded, FromPrimitive, Integer, ToPrimitive};
use serde::Deserializer;

mod float;
mod integer;

pub use float::*;
pub use integer::*;

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

pub fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}
