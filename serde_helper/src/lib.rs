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

struct InfFloatVisitor;

impl<'de> serde::de::Visitor<'de> for InfFloatVisitor {
    type Value = f64;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(
            "a double precision floating point number or a string containing \"inf\", \"-inf\" or \"NaN\"",
        )
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value)
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(f64::from(v))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value as f64)
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value as f64)
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            "inf" => Ok(f64::INFINITY),
            "-inf" => Ok(f64::NEG_INFINITY),
            "NaN" => Ok(f64::NAN),
            _ => Err(E::custom(format!(
                "invalid string for special float value: {value}"
            ))),
        }
    }
}

#[allow(clippy::missing_errors_doc)]
pub fn inf_float_deserializer<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(InfFloatVisitor)
}

#[allow(clippy::missing_errors_doc)]
pub fn inf_float_serializer<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if value.is_nan() {
        serializer.serialize_str("NaN")
    } else if value.is_infinite() {
        if value.is_sign_positive() {
            serializer.serialize_str("inf")
        } else {
            serializer.serialize_str("-inf")
        }
    } else {
        serializer.serialize_f64(*value)
    }
}

struct InfFloatOptVisitor;

impl<'de> serde::de::Visitor<'de> for InfFloatOptVisitor {
    type Value = Option<f64>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(
            "a double precision floating point number or a string containing \"inf\", \"-inf\" or \"NaN\"",
        )
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_any(InfFloatVisitor).map(Some)
    }
}

#[allow(clippy::missing_errors_doc)]
pub fn inf_float_opt_deserializer<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_option(InfFloatOptVisitor)
}

#[allow(clippy::missing_errors_doc)]
pub fn inf_float_opt_serializer<S>(value: &Option<f64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if let Some(value) = value {
        inf_float_serializer(value, serializer)
    } else {
        serializer.serialize_none()
    }
}

#[must_use]
pub const fn bool_true() -> bool {
    true
}

pub fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}
