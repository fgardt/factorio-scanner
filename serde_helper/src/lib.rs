use num::{Bounded, FromPrimitive, Integer, ToPrimitive};
use serde::Deserializer;

mod float;
mod integer;

pub use float::*;
pub use integer::*;

struct TruncatingVisitor<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> serde::de::Visitor<'_> for TruncatingVisitor<T>
where
    T: Bounded + Integer + ToPrimitive + FromPrimitive,
{
    type Value = T;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a integer value")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_f64(v as f64)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_f64(v as f64)
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let Some(min) = T::min_value().to_f64() else {
            return Err(E::custom(format!(
                "unable to convert {} min value to f64",
                std::any::type_name::<T>()
            )));
        };
        let Some(max) = T::max_value().to_f64() else {
            return Err(E::custom(format!(
                "unable to convert {} max value to f64",
                std::any::type_name::<T>()
            )));
        };

        if v < min || v > max {
            return Err(E::custom(format!(
                "{} out of range: {}",
                std::any::type_name::<T>(),
                v,
            )));
        }

        T::from_f64(v).map_or_else(
            || {
                Err(E::custom(format!(
                    "unable to truncate {} to {}",
                    v,
                    std::any::type_name::<T>(),
                )))
            },
            |res| Ok(res),
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v.parse::<f64>() {
            Ok(v) => self.visit_f64(v),
            Err(err) => Err(E::invalid_type(
                serde::de::Unexpected::Other(&err.to_string()),
                &self,
            )),
        }
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(&v)
    }
}

#[allow(clippy::missing_errors_doc)]
pub fn truncating_deserializer<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Bounded + Integer + ToPrimitive + FromPrimitive,
{
    deserializer.deserialize_any(TruncatingVisitor {
        _marker: std::marker::PhantomData,
    })
}

#[allow(clippy::missing_errors_doc)]
pub fn truncating_opt_deserializer<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Bounded + Integer + ToPrimitive + FromPrimitive,
{
    use serde::{Deserialize, de::Error as deError};

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

impl serde::de::Visitor<'_> for InfFloatVisitor {
    type Value = f32;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(
            "a single precision floating point number or a string containing \"inf\", \"-inf\" or \"NaN\"",
        )
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value as f32)
    }

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value)
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value as f32)
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value as f32)
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            "inf" => Ok(f32::INFINITY),
            "-inf" => Ok(f32::NEG_INFINITY),
            "NaN" => Ok(f32::NAN),
            _ => Err(E::custom(format!(
                "invalid string for special float value: {value}"
            ))),
        }
    }
}

#[allow(clippy::missing_errors_doc)]
pub fn inf_float_deserializer<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(InfFloatVisitor)
}

#[allow(clippy::missing_errors_doc)]
pub fn inf_float_serializer<S>(value: &f32, serializer: S) -> Result<S::Ok, S::Error>
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
        serializer.serialize_f32(*value)
    }
}

struct InfFloatOptVisitor;

impl<'de> serde::de::Visitor<'de> for InfFloatOptVisitor {
    type Value = Option<f32>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(
            "a single precision floating point number or a string containing \"inf\", \"-inf\" or \"NaN\"",
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
pub fn inf_float_opt_deserializer<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_option(InfFloatOptVisitor)
}

#[allow(clippy::missing_errors_doc)]
pub fn inf_float_opt_serializer<S>(value: &Option<f32>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if let Some(value) = value {
        inf_float_serializer(value, serializer)
    } else {
        serializer.serialize_none()
    }
}

struct InfDoubleVisitor;

impl serde::de::Visitor<'_> for InfDoubleVisitor {
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
pub fn inf_double_deserializer<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(InfDoubleVisitor)
}

#[allow(clippy::missing_errors_doc)]
pub fn inf_double_serializer<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
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

struct InfDoubleOptVisitor;

impl<'de> serde::de::Visitor<'de> for InfDoubleOptVisitor {
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
        deserializer.deserialize_any(InfDoubleVisitor).map(Some)
    }
}

#[allow(clippy::missing_errors_doc)]
pub fn inf_double_opt_deserializer<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_option(InfDoubleOptVisitor)
}

#[allow(clippy::missing_errors_doc)]
pub fn inf_double_opt_serializer<S>(value: &Option<f64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if let Some(value) = value {
        inf_double_serializer(value, serializer)
    } else {
        serializer.serialize_none()
    }
}

struct BoolVisitor;

impl serde::de::Visitor<'_> for BoolVisitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a boolean value")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value)
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value != 0)
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value != 0)
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        value.parse::<i64>().map_or_else(
            |_| match value {
                "true" => Ok(true),
                "false" | "" => Ok(false),
                _ => Err(E::custom(format!(
                    "invalid string for boolean value: {value}"
                ))),
            },
            |num| self.visit_i64(num),
        )
    }
}

#[allow(clippy::missing_errors_doc)]
pub fn bool_deserializer<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(BoolVisitor)
}

#[must_use]
pub const fn bool_true() -> bool {
    true
}

pub fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}
