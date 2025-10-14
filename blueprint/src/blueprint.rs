use std::collections::HashMap;

use serde::{
    Deserialize, Serialize,
    de::{IntoDeserializer, Visitor},
    ser::SerializeSeq,
};
use serde_helper as helper;
use serde_with::skip_serializing_none;

use mod_util::{AnyBasic, DependencyList, mod_info::DependencyVersion};
use types::{
    AsteroidChunkID, Comparator, EntityID, FluidID, ItemID, QualityID, RecipeID, SpaceLocationID,
    TileID, VirtualSignalID,
};

use crate::IndexedVec;

mod entity;
mod logistics;
mod parameters;
mod trains;

pub use entity::*;
pub use logistics::*;
pub use parameters::*;
pub use trains::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BlueprintData {
    #[serde(flatten)]
    pub snapping: SnapData,

    pub icons: IndexedVec<Icon>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entities: Vec<Entity>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tiles: Vec<Tile>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schedules: Vec<Schedule>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stock_connections: Vec<StockConnection>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub wires: Vec<WireData>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ParameterData>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
}

impl crate::GetIDs for BlueprintData {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = self.icons.get_ids();

        ids.merge(self.entities.get_ids());

        for tile in &self.tiles {
            ids.tile.insert(tile.name.clone());
        }

        ids.merge(self.schedules.get_ids());

        ids
    }
}

impl BlueprintData {
    #[must_use]
    pub fn has_meta_info(&self) -> bool {
        self.entities
            .iter()
            .any(|e| e.tags.contains_key("bp_meta_info"))
    }

    #[must_use]
    pub fn get_meta_info_mods(&self) -> Option<DependencyList> {
        for e in &self.entities {
            let Some(info) = e.tags.get("bp_meta_info") else {
                continue;
            };

            let AnyBasic::Table(data) = info else {
                continue;
            };

            let Some(mods) = data.get("mods") else {
                continue;
            };

            let AnyBasic::Table(mods) = mods else {
                continue;
            };

            let mut result = HashMap::with_capacity(mods.len());

            for (mod_name, mod_version) in mods {
                let AnyBasic::String(mod_version) = mod_version else {
                    continue;
                };

                let Ok(version) = mod_version.try_into() else {
                    continue;
                };

                result.insert(mod_name.clone(), DependencyVersion::Exact(version));
            }

            return Some(result);
        }

        None
    }
}

pub type Blueprint = crate::CommonData<BlueprintData>;

#[skip_serializing_none]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct SnapData {
    pub snap_to_grid: Option<Position>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub absolute_snapping: bool,

    pub position_relative_to_grid: Option<Position>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Icon {
    pub signal: SignalID,
}

impl crate::GetIDs for Icon {
    fn get_ids(&self) -> crate::UsedIDs {
        self.signal.get_ids()
    }
}

/// [`SignalID`](https://lua-api.factorio.com/latest/concepts/SignalID.html)
#[skip_serializing_none]
#[derive(Debug, Clone, /*Deserialize,*/ Serialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum SignalID {
    Item {
        name: Option<ItemID>,
        quality: Option<QualityID>,
    },
    Fluid {
        name: Option<FluidID>,
        quality: Option<QualityID>,
    },
    Virtual {
        name: Option<VirtualSignalID>,
        quality: Option<QualityID>,
    },
    Entity {
        name: Option<EntityID>,
        quality: Option<QualityID>,
    },
    Recipe {
        name: Option<RecipeID>,
        quality: Option<QualityID>,
    },
    SpaceLocation {
        name: Option<SpaceLocationID>,
        quality: Option<QualityID>,
    },
    AsteroidChunk {
        name: Option<AsteroidChunkID>,
        quality: Option<QualityID>,
    },
    Quality {
        name: Option<QualityID>,
    },
}

impl SignalID {
    #[must_use]
    pub fn name(&self) -> Option<String> {
        match self {
            Self::Item { name, .. } => name.clone().map(|n| (*n).clone()),
            Self::Fluid { name, .. } => name.clone().map(|n| (*n).clone()),
            Self::Virtual { name, .. } => name.clone().map(|n| (*n).clone()),
            Self::Entity { name, .. } => name.clone().map(|n| (*n).clone()),
            Self::Recipe { name, .. } => name.clone().map(|n| (*n).clone()),
            Self::SpaceLocation { name, .. } => name.clone().map(|n| (*n).clone()),
            Self::AsteroidChunk { name, .. } => name.clone().map(|n| (*n).clone()),
            Self::Quality { name } => name.clone().map(|n| (*n).clone()),
        }
    }

    #[must_use]
    pub const fn quality(&self) -> &Option<QualityID> {
        match self {
            Self::Item { quality, .. }
            | Self::Fluid { quality, .. }
            | Self::Virtual { quality, .. }
            | Self::Entity { quality, .. }
            | Self::Recipe { quality, .. }
            | Self::SpaceLocation { quality, .. }
            | Self::AsteroidChunk { quality, .. } => quality,
            Self::Quality { .. } => &None,
        }
    }
}

impl crate::GetIDs for SignalID {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = crate::UsedIDs::default();

        if self.name().is_some() {
            match self {
                Self::Item { name, .. } => ids.item.insert(name.clone().unwrap_or_default()),
                Self::Fluid { name, .. } => ids.fluid.insert(name.clone().unwrap_or_default()),
                Self::Virtual { name, .. } => {
                    ids.virtual_signal.insert(name.clone().unwrap_or_default())
                }
                Self::Entity { name, .. } => ids.entity.insert(name.clone().unwrap_or_default()),
                Self::Recipe { name, .. } => ids.recipe.insert(name.clone().unwrap_or_default()),
                Self::SpaceLocation { name, .. } => {
                    ids.space_location.insert(name.clone().unwrap_or_default())
                }
                Self::AsteroidChunk { name, .. } => {
                    ids.asteroid_chunk.insert(name.clone().unwrap_or_default())
                }
                Self::Quality { name } => ids.quality.insert(name.clone().unwrap_or_default()),
            };
        }

        if let Some(quality) = self.quality() {
            ids.quality.insert(quality.clone());
        }

        ids
    }
}

impl<'de> Deserialize<'de> for SignalID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SignalIDVisitor;

        impl<'de> Visitor<'de> for SignalIDVisitor {
            type Value = SignalID;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a SignalID")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut kind = None;
                let mut name = None;
                let mut quality = None;

                while let Ok(Some((key, value))) = map.next_entry::<String, String>() {
                    match key.as_str() {
                        "type" => {
                            kind = Some(SignalIDType::deserialize(value.into_deserializer())?);
                        }
                        "name" => {
                            name = Some(value);
                        }
                        "quality" => {
                            quality = Some(QualityID::new(value));
                        }
                        _ => {}
                    }
                }

                match kind.unwrap_or_default() {
                    SignalIDType::Item => {
                        let name = name.map(ItemID::new);
                        Ok(SignalID::Item { name, quality })
                    }
                    SignalIDType::Fluid => {
                        let name = name.map(FluidID::new);
                        Ok(SignalID::Fluid { name, quality })
                    }
                    SignalIDType::Virtual => {
                        let name = name.map(VirtualSignalID::new);
                        Ok(SignalID::Virtual { name, quality })
                    }
                    SignalIDType::Entity => {
                        let name = name.map(EntityID::new);
                        Ok(SignalID::Entity { name, quality })
                    }
                    SignalIDType::Recipe => {
                        let name = name.map(RecipeID::new);
                        Ok(SignalID::Recipe { name, quality })
                    }
                    SignalIDType::SpaceLocation => {
                        let name = name.map(SpaceLocationID::new);
                        Ok(SignalID::SpaceLocation { name, quality })
                    }
                    SignalIDType::AsteroidChunk => {
                        let name = name.map(AsteroidChunkID::new);
                        Ok(SignalID::AsteroidChunk { name, quality })
                    }
                    SignalIDType::Quality => {
                        let name = name.map(QualityID::new);
                        Ok(SignalID::Quality { name })
                    }
                }
            }
        }

        deserializer.deserialize_map(SignalIDVisitor)
    }
}

/// [`SignalIDType`](https://lua-api.factorio.com/latest/concepts/SignalIDType.html)
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SignalIDType {
    #[default]
    Item,
    Fluid,
    Virtual,
    Entity,
    Recipe,
    SpaceLocation,
    AsteroidChunk,
    Quality,
}

/// [`BlueprintWire`](https://lua-api.factorio.com/latest/concepts/BlueprintWire.html)
// todo: use defines.wire_connector_id
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WireData {
    pub source_entity: EntityNumber,
    pub source_connector: u8,
    pub target_entity: EntityNumber,
    pub target_connector: u8,
}

impl Serialize for WireData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(4))?;
        seq.serialize_element(&self.source_entity)?;
        seq.serialize_element(&self.source_connector)?;
        seq.serialize_element(&self.target_entity)?;
        seq.serialize_element(&self.target_connector)?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for WireData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct WireDataVisitor;

        impl<'de> Visitor<'de> for WireDataVisitor {
            type Value = WireData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("wire connection data")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let source_entity = seq.next_element()?;
                let source_connector = seq.next_element()?;
                let target_entity = seq.next_element()?;
                let target_connector = seq.next_element()?;
                let end = seq.next_element::<Option<()>>()?;

                let (
                    Some(source_entity),
                    Some(source_connector),
                    Some(target_entity),
                    Some(target_connector),
                    None,
                ) = (
                    source_entity,
                    source_connector,
                    target_entity,
                    target_connector,
                    end,
                )
                else {
                    return Err(serde::de::Error::custom(
                        "wire connection data needs 4 elements",
                    ));
                };

                Ok(WireData {
                    source_entity,
                    source_connector,
                    target_entity,
                    target_connector,
                })
            }
        }

        deserializer.deserialize_seq(WireDataVisitor)
    }
}

/// [`CircuitCondition`](https://lua-api.factorio.com/latest/concepts/CircuitCondition.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged, deny_unknown_fields)]
pub enum CircuitCondition {
    Signals {
        comparator: Comparator,
        first_signal: Option<SignalID>,
        second_signal: SignalID,
    },
    Constant {
        comparator: Comparator,
        first_signal: Option<SignalID>,
        #[serde(default)]
        constant: i32,
    },
}

impl crate::GetIDs for CircuitCondition {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = crate::UsedIDs::default();

        match self {
            Self::Signals {
                first_signal,
                second_signal,
                ..
            } => {
                ids.merge(first_signal.get_ids());
                ids.merge(second_signal.get_ids());
            }
            Self::Constant { first_signal, .. } => {
                ids.merge(first_signal.get_ids());
            }
        }

        ids
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct RequestCondition {
    pub name: ItemID,
    pub quality: Option<QualityID>,
}

impl crate::GetIDs for RequestCondition {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = self.name.get_ids();
        ids.merge(self.quality.get_ids());

        ids
    }
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CompareType {
    And,
    #[default]
    Or,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Tile {
    pub name: TileID,
    pub position: Position,
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.position.partial_cmp(&other.position)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Position {
    #[serde(serialize_with = "shorter_floats")]
    pub x: f32,

    #[serde(serialize_with = "shorter_floats")]
    pub y: f32,
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn shorter_floats<S>(x: &f32, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    // serialize as integer if possible
    if x.rem_euclid(1.0) == 0.0 {
        #[allow(clippy::cast_possible_truncation)]
        s.serialize_i32(*x as i32)
    } else {
        s.serialize_f32(*x)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.y.partial_cmp(&other.y).map_or_else(
            || self.x.partial_cmp(&other.x),
            |res| match res {
                std::cmp::Ordering::Equal => self.x.partial_cmp(&other.x),
                std::cmp::Ordering::Less | std::cmp::Ordering::Greater => Some(res),
            },
        )
    }
}

impl From<Position> for types::MapPosition {
    fn from(value: Position) -> Self {
        Self::XY {
            x: f64::from(value.x),
            y: f64::from(value.y),
        }
    }
}

impl From<&Position> for types::MapPosition {
    fn from(value: &Position) -> Self {
        Self::XY {
            x: f64::from(value.x),
            y: f64::from(value.y),
        }
    }
}

impl From<Position> for types::Vector {
    fn from(value: Position) -> Self {
        Self::Tuple(value.x.into(), value.y.into())
    }
}

impl From<&Position> for types::Vector {
    fn from(value: &Position) -> Self {
        Self::Tuple(value.x.into(), value.y.into())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl From<&Color> for types::Color {
    fn from(value: &Color) -> Self {
        Self::RGBA(value.r, value.g, value.b, value.a)
    }
}

/// [`QualityCondition`](https://lua-api.factorio.com/latest/concepts/QualityCondition.html)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QualityCondition {
    pub quality: Option<QualityID>,
    pub comparator: Option<Comparator>,
}
