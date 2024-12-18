use std::ops::{Deref, DerefMut};

use serde::{
    de::Visitor,
    ser::{SerializeMap, SerializeSeq},
    Deserialize, Serialize,
};

/// <https://forums.factorio.com/viewtopic.php?t=109077>
#[derive(Debug, Clone)]
pub struct FactorioArray<T>(Vec<T>);

impl<T> FactorioArray<T> {
    #[must_use]
    pub const fn new(data: Vec<T>) -> Self {
        Self(data)
    }
}

impl<T> Deref for FactorioArray<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for FactorioArray<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> AsRef<Vec<T>> for FactorioArray<T> {
    fn as_ref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T> AsMut<Vec<T>> for FactorioArray<T> {
    fn as_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
}

impl<T> Default for FactorioArray<T> {
    fn default() -> Self {
        Self(Vec::with_capacity(0))
    }
}

impl<T> IntoIterator for FactorioArray<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a FactorioArray<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<T> From<Vec<T>> for FactorioArray<T> {
    fn from(vec: Vec<T>) -> Self {
        Self(vec)
    }
}

impl<T> From<FactorioArray<T>> for Vec<T> {
    fn from(array: FactorioArray<T>) -> Self {
        array.0
    }
}

impl<'a, T> From<&'a FactorioArray<T>> for &'a Vec<T> {
    fn from(array: &'a FactorioArray<T>) -> Self {
        &array.0
    }
}

impl<T: Serialize> Serialize for FactorioArray<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.is_empty() {
            serializer.serialize_map(Some(0))?.end()
        } else {
            let mut s = serializer.serialize_seq(Some(self.len()))?;
            for elem in self {
                s.serialize_element(&elem)?;
            }
            s.end()
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ArrayEntry<T> {
    Some(T),
    None {},
}

struct FactorioArrayVisitor<T> {
    marker: std::marker::PhantomData<T>,
}

impl<T> FactorioArrayVisitor<T> {
    const fn new() -> Self {
        Self {
            marker: std::marker::PhantomData,
        }
    }
}

impl<'de, T> Visitor<'de> for FactorioArrayVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = FactorioArray<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a sequence or an empty map")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut vec = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(elem) = seq.next_element::<ArrayEntry<T>>()? {
            if let ArrayEntry::Some(elem) = elem {
                vec.push(elem);
            }
        }

        Ok(FactorioArray(vec))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut vec = Vec::<(String, T)>::with_capacity(map.size_hint().unwrap_or(0));

        while let Some((key, value)) = map.next_entry::<String, ArrayEntry<T>>()? {
            if let ArrayEntry::Some(value) = value {
                vec.push((key, value));
            }
        }

        vec.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

        Ok(FactorioArray(vec.into_iter().map(|(_, v)| v).collect()))
    }
}

impl<'de, T> Deserialize<'de> for FactorioArray<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(FactorioArrayVisitor::new())
    }
}
