#![forbid(unsafe_code)]
#![allow(
    unused_variables,
    clippy::wildcard_imports,
    clippy::struct_excessive_bools,
    clippy::module_name_repetitions
)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;

use serde_helper as helper;

mod energy;
mod graphics;
mod icon;

pub use energy::*;
pub use graphics::*;
pub use icon::*;

///[`Types/Color`](https://lua-api.factorio.com/latest/types/Color.html)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Color {
    Struct {
        r: Option<f64>,
        g: Option<f64>,
        b: Option<f64>,
        a: Option<f64>,
    },
    RGB(f64, f64, f64),
    RGBA(f64, f64, f64, f64),
}

impl Color {
    pub fn to_rgba(&self) -> [f64; 4] {
        let (r, g, b, a) = match self {
            Self::Struct { r, g, b, a } => (*r, *g, *b, *a),
            Self::RGB(r, g, b) => (Some(*r), Some(*g), Some(*b), None::<f64>),
            Self::RGBA(r, g, b, a) => (Some(*r), Some(*g), Some(*b), Some(*a)),
        };

        let r = r.unwrap_or(0.0);
        let g = g.unwrap_or(0.0);
        let b = b.unwrap_or(0.0);

        if r > 1.0 || g > 1.0 || b > 1.0 {
            let a = a.unwrap_or(255.0);

            [r / 255.0, g / 255.0, b / 255.0, a / 255.0]
        } else {
            let a = a.unwrap_or(1.0);

            [r, g, b, a]
        }
    }

    pub const fn white() -> Self {
        Self::RGBA(1.0, 1.0, 1.0, 1.0)
    }

    pub fn is_white(color: &Self) -> bool {
        let [r, g, b, a] = color.to_rgba();

        r == 1.0 && g == 1.0 && b == 1.0 && a == 1.0
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::RGBA(0.0, 0.0, 0.0, 1.0)
    }
}

/// [`Types/DefaultRecipeTint`](https://lua-api.factorio.com/latest/types/DefaultRecipeTint.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultRecipeTint {
    #[serde(default = "Color::white", skip_serializing_if = "Color::is_white")]
    pub primary: Color,

    #[serde(default = "Color::white", skip_serializing_if = "Color::is_white")]
    pub secondary: Color,

    #[serde(default = "Color::white", skip_serializing_if = "Color::is_white")]
    pub tertiary: Color,

    #[serde(default = "Color::white", skip_serializing_if = "Color::is_white")]
    pub quaternary: Color,
}

/// [`Types/StatusColors`](https://lua-api.factorio.com/latest/types/StatusColors.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct StatusColors {
    #[serde(default = "Color::white", skip_serializing_if = "Color::is_white")]
    pub idle: Color,
    pub no_minable_resources: Option<Color>,
    pub full_output: Option<Color>,
    pub insufficient_input: Option<Color>,
    pub disabled: Option<Color>,
    pub no_power: Option<Color>,

    #[serde(default = "Color::white", skip_serializing_if = "Color::is_white")]
    pub working: Color,
    pub low_power: Option<Color>,
}

/// [`Types/Vector`](https://lua-api.factorio.com/latest/types/Vector.html)
pub type Vector = (f64, f64);

/// [`Types/Vector3D`](https://lua-api.factorio.com/latest/types/Vector3D.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Vector3D {
    Struct { x: f64, y: f64, z: f64 },
    Tuple(f64, f64, f64),
}

/// [`Types/WirePosition`](https://lua-api.factorio.com/latest/types/WirePosition.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct WirePosition {
    pub copper: Option<Vector>,
    pub green: Option<Vector>,
    pub red: Option<Vector>,
}

/// [`Types/WireConnectionPoint`](https://lua-api.factorio.com/latest/types/WireConnectionPoint.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct WireConnectionPoint {
    pub wire: WirePosition,
    pub shadow: WirePosition,
}

/// [`Types/FileName`](https://lua-api.factorio.com/latest/types/FileName.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileName(String);

pub type ImageCache = HashMap<String, Option<image::DynamicImage>>;

impl FileName {
    pub fn get(&self) -> &str {
        self.0.as_str()
    }

    pub fn load<'a>(
        &self,
        factorio_dir: &str,
        used_mods: &HashMap<&str, &str>,
        image_cache: &'a mut ImageCache,
    ) -> Option<&'a image::DynamicImage> {
        const VANILLA_MODS: [&str; 2] = ["core", "base"];
        let filename = self.get();

        if image_cache.contains_key(&filename.to_owned()) {
            return image_cache.get(filename)?.as_ref();
        }

        let re = regex::Regex::new(r"^__([^/\\]+)__").ok()?;
        let mod_name = re.captures(filename)?.get(1)?.as_str();
        let sprite_path = &filename[(2 + mod_name.len() + 2 + 1)..]; // +1 to include the slash to prevent joining to interpret it as a absolute path

        let img = if VANILLA_MODS.contains(&mod_name) {
            let location = std::path::Path::new(factorio_dir)
                .join("data")
                .join(mod_name)
                .join(sprite_path);

            image::open(location).ok()
        } else {
            // TODO: support unzipped mods?

            let mod_version = used_mods.get(&mod_name)?;
            let mod_zip_path = std::path::Path::new(factorio_dir)
                .join("mods")
                .join(format!("{mod_name}_{mod_version}.zip"));

            let mod_zip_file = std::fs::File::open(mod_zip_path).ok()?;
            let mut zip = zip::ZipArchive::new(mod_zip_file).ok()?;

            if zip.is_empty() {
                return None;
            }

            // TODO: this could break if there are files in the root of the zip alongside the mod folder
            let internal_mod_folder;
            {
                let extractor_file = &zip.by_index(0).ok()?;
                let re = regex::Regex::new(r"^([^/]+)/").ok()?;
                internal_mod_folder = re
                    .captures(extractor_file.name())?
                    .get(1)?
                    .as_str()
                    .to_owned();
            }

            let location = filename.replace(
                format!("__{mod_name}__").as_str(),
                internal_mod_folder.as_str(),
            );
            let mut file = zip.by_name(location.as_str()).ok()?;

            let mut file_buff = Vec::new();
            std::io::Read::read_to_end(&mut file, &mut file_buff).ok()?;

            image::load_from_memory(&file_buff).ok()
        };

        image_cache.insert(filename.to_owned(), img.clone());
        image_cache.get(&filename.to_owned())?.as_ref()
    }
}

/// [`Types/LocalisedString`](https://lua-api.factorio.com/latest/types/LocalisedString.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LocalisedString {
    Bool(bool),
    String(String),
    Array(Vec<LocalisedString>),
}

/// [`Types/Order`](https://lua-api.factorio.com/latest/types/Order.html)
pub type Order = String;

/// [`Types/RealOrientation`](https://lua-api.factorio.com/latest/types/RealOrientation.html)
pub type RealOrientation = f64;

/// [`Types/FuelCategoryID`](https://lua-api.factorio.com/latest/types/FuelCategoryID.html)
pub type FuelCategoryID = String;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FuelCategory {
    Single {
        // TODO: handle default value of: `chemical`
        fuel_category: FuelCategoryID,
    },
    Multi {
        fuel_categories: Vec<FuelCategoryID>,
    },
}

/// [`Types/VirtualSignalID`](https://lua-api.factorio.com/latest/types/VirtualSignalID.html)
pub type VirtualSignalID = String;

/// [`Types/FluidID`](https://lua-api.factorio.com/latest/types/FluidID.html)
pub type FluidID = String;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PipeConnectionType {
    #[default]
    InputOutput,
    Input,
    Output,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PipeConnectionDefinition {
    Multi {
        positions: Vec<Vector>,

        #[serde(
            default,
            skip_serializing_if = "helper::is_0_u32",
            deserialize_with = "helper::truncating_deserializer"
        )]
        max_underground_distance: u32,
    },
    Single {
        position: Vector,

        #[serde(
            default,
            skip_serializing_if = "helper::is_0_u32",
            deserialize_with = "helper::truncating_deserializer"
        )]
        max_underground_distance: u32,

        #[serde(default, rename = "type")]
        type_: PipeConnectionType,
    },
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FluidBoxProductionType {
    #[default]
    None,
    #[serde(rename = "None")]
    None2,
    Input,
    InputOutput,
    Output,
}

/// [`Types/FluidBox.secondary_draw_orders`](https://lua-api.factorio.com/latest/types/FluidBox.html#secondary_draw_orders)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FluidBoxSecondaryDrawOrders {
    Global {
        #[serde(
            default = "helper::i8_1",
            skip_serializing_if = "helper::is_1_i8",
            deserialize_with = "helper::truncating_deserializer"
        )]
        secondary_draw_order: i8,
    },
    Cardinal {
        #[serde(
            default = "helper::i8_1",
            skip_serializing_if = "helper::is_1_i8",
            deserialize_with = "helper::truncating_deserializer"
        )]
        north: i8,

        #[serde(
            default = "helper::i8_1",
            skip_serializing_if = "helper::is_1_i8",
            deserialize_with = "helper::truncating_deserializer"
        )]
        east: i8,

        #[serde(
            default = "helper::i8_1",
            skip_serializing_if = "helper::is_1_i8",
            deserialize_with = "helper::truncating_deserializer"
        )]
        south: i8,

        #[serde(
            default = "helper::i8_1",
            skip_serializing_if = "helper::is_1_i8",
            deserialize_with = "helper::truncating_deserializer"
        )]
        west: i8,
    },
}

/// [`Types/FluidBox`](https://lua-api.factorio.com/latest/types/FluidBox.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct FluidBox {
    pub pipe_connections: EmptyArrayFix<PipeConnectionDefinition>,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub base_area: f64,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub base_level: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub height: f64,

    pub filter: Option<FluidID>,
    pub render_layer: Option<RenderLayer>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub hide_connection_info: bool,

    pub pipe_covers: Option<Sprite4Way>,
    pub pipe_picture: Option<Sprite4Way>,

    pub minimum_temperature: Option<f64>,
    pub maximum_temperature: Option<f64>,

    // TODO: skip serializing if default
    #[serde(default)]
    pub production_type: FluidBoxProductionType,

    #[serde(flatten)]
    pub secondary_draw_order: Option<FluidBoxSecondaryDrawOrders>,
}

/// [`Types/RecipeID`](https://lua-api.factorio.com/latest/types/RecipeID.html)
pub type RecipeID = String;

/// [`Types/RecipeCategoryID`](https://lua-api.factorio.com/latest/types/RecipeCategoryID.html)
pub type RecipeCategoryID = String;

/// [`Types/AmmoCategoryID`](https://lua-api.factorio.com/latest/types/AmmoCategoryID.html)
pub type AmmoCategoryID = String;

/// [`Types/EquipmentGridID`](https://lua-api.factorio.com/latest/types/EquipmentGridID.html)
pub type EquipmentGridID = String;

/// [`Types/ResourceCategoryID`](https://lua-api.factorio.com/latest/types/ResourceCategoryID.html)
pub type ResourceCategoryID = String;

/// [`Types/ItemID`](https://lua-api.factorio.com/latest/types/ItemID.html)
pub type ItemID = String;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SignalIDConnector {
    Virtual { name: VirtualSignalID },
    Item { name: ItemID },
    Fluid { name: FluidID },
}

/// [`Types/ItemStackIndex`](https://lua-api.factorio.com/latest/types/ItemStackIndex.html)
pub type ItemStackIndex = u16;

/// [`Types/ItemCountType`](https://lua-api.factorio.com/latest/types/ItemCountType.html)
pub type ItemCountType = u32;

/// [`Types/ItemSubGroupID`](https://lua-api.factorio.com/latest/types/ItemSubGroupID.html)
pub type ItemSubGroupID = String;

/// [`Types/ItemToPlace`](https://lua-api.factorio.com/latest/types/ItemToPlace.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemToPlace {
    pub item: ItemID,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PlaceableBy {
    Single(ItemToPlace),
    Multiple(Vec<ItemToPlace>),
}

/// [`Types/ModuleSpecification`](https://lua-api.factorio.com/latest/types/ModuleSpecification.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleSpecification {
    #[serde(
        default,
        skip_serializing_if = "helper::is_0_u16",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub module_slots: ItemStackIndex,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "helper::truncating_opt_deserializer"
    )]
    pub module_info_max_icons_per_row: Option<u8>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "helper::truncating_opt_deserializer"
    )]
    pub module_info_max_icon_rows: Option<u8>,
    pub module_info_icon_shift: Option<Vector>,
    pub module_info_separation_multiplier: Option<f32>,
    pub module_info_multi_row_initial_height_modifier: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EffectType {
    Speed,
    Productivity,
    Consumption,
    Pollution,
}

/// [`Types/EffectTypeLimitation`](https://lua-api.factorio.com/latest/types/EffectTypeLimitation.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EffectTypeLimitation {
    Single(EffectType),
    Multiple(EmptyArrayFix<EffectType>),
}

/// [`Types/CollisionMask`](https://lua-api.factorio.com/latest/types/CollisionMask.html)
pub type CollisionMask = EmptyArrayFix<String>;

/// [`Types/EntityID`](https://lua-api.factorio.com/latest/types/EntityID.html)
pub type EntityID = String;

/// Union used in [`Types/EntityPrototypeFlags`](https://lua-api.factorio.com/latest/types/EntityPrototypeFlags.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EntityPrototypeFlag {
    NotRotatable,
    PlaceableNeutral,
    PlaceablePlayer,
    PlaceableEnemy,
    PlaceableOffGrid,
    PlayerCreation,
    #[serde(rename = "building-direction-8-way")]
    BuildingDirection8Way,
    FilterDirections,
    FastReplaceableNoBuildWhileMoving,
    BreathsAir,
    NotRepairable,
    NotOnMap,
    NotDeconstructable,
    NotBlueprintable,
    Hidden,
    HideAltInfo,
    FastReplaceNoCrossTypeWhileMoving,
    NoGapFillWhileBuilding,
    NotFlammable,
    NoAutomatedItemRemoval,
    NoAutomatedItemInsertion,
    NoCopyPaste,
    NotSelectableInGame,
    NotUpgradable,
    NotInKillStatistics,
    NotInMadeIn,
}

/// <https://forums.factorio.com/viewtopic.php?t=109077>
#[allow(clippy::zero_sized_map_values)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmptyArrayFix<T> {
    Flags(Vec<T>),
    Empty(HashMap<(), ()>),
}

/// [`Types/EntityPrototypeFlags`](https://lua-api.factorio.com/latest/types/EntityPrototypeFlags.html)
// pub type EntityPrototypeFlags = Vec<EntityPrototypeFlag>;

/// [`Types/EntityPrototypeFlags`](https://lua-api.factorio.com/latest/types/EntityPrototypeFlags.html)
pub type EntityPrototypeFlags = EmptyArrayFix<EntityPrototypeFlag>;

/// [`Types/MapPosition`](https://lua-api.factorio.com/latest/types/MapPosition.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MapPosition {
    XY { x: f64, y: f64 },
    Tuple(f64, f64),
}

impl MapPosition {
    pub const fn as_tuple(&self) -> (f64, f64) {
        match self {
            Self::Tuple(x, y) | Self::XY { x, y } => (*x, *y),
        }
    }

    pub fn is_close(&self, other: &Self, distance: f64) -> bool {
        const EPSILON: f64 = 0.125;

        let (x1, y1) = self.as_tuple();
        let (x2, y2) = other.as_tuple();

        (x1 - x2).abs() < EPSILON && (y1 - y2).abs() < EPSILON
    }

    fn is_cardinal_neighbor_internal(
        &self,
        other: &Self,
        cardinal_max: f64,
        cardinal_min: f64,
        shear_max: f64,
    ) -> Option<Direction> {
        let (x1, y1) = self.as_tuple();
        let (x2, y2) = other.as_tuple();
        let x_diff = (x1 - x2).abs();
        let y_diff = (y1 - y2).abs();

        if x_diff < cardinal_max && x_diff > cardinal_min && y_diff < shear_max {
            if x1 > x2 {
                Some(Direction::West)
            } else {
                Some(Direction::East)
            }
        } else if y_diff < cardinal_max && y_diff > cardinal_min && x_diff < shear_max {
            if y1 > y2 {
                Some(Direction::North)
            } else {
                Some(Direction::South)
            }
        } else {
            None
        }
    }

    pub fn is_cardinal_neighbor(&self, other: &Self) -> Option<Direction> {
        const CARDINAL_MAX: f64 = 1.125;
        const CARDINAL_MIN: f64 = 0.875;
        const SHEAR_MAX: f64 = 0.125;

        self.is_cardinal_neighbor_internal(other, CARDINAL_MAX, CARDINAL_MIN, SHEAR_MAX)
    }

    pub fn is_2long_cardinal_neighbor(&self, other: &Self) -> Option<Direction> {
        const CARDINAL_MAX: f64 = 2.125;
        const CARDINAL_MIN: f64 = 1.875;
        const SHEAR_MAX: f64 = 0.125;

        self.is_cardinal_neighbor_internal(other, CARDINAL_MAX, CARDINAL_MIN, SHEAR_MAX)
    }

    pub fn is_2wide_cardinal_neighbor(&self, other: &Self) -> Option<Direction> {
        const CARDINAL_MAX: f64 = 1.125;
        const CARDINAL_MIN: f64 = 0.875;
        const SHEAR_MAX: f64 = 0.875;

        self.is_cardinal_neighbor_internal(other, CARDINAL_MAX, CARDINAL_MIN, SHEAR_MAX)
    }
}

/// [`Types/BoundingBox`](https://lua-api.factorio.com/latest/types/BoundingBox.html)
pub type BoundingBox = (MapPosition, MapPosition);

/// [`Types/Direction`](https://lua-api.factorio.com/latest/types/Direction.html)
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize_repr, Deserialize_repr,
)]
#[repr(u8)]
pub enum Direction {
    #[default]
    North = 0,
    NorthEast = 1,
    East = 2,
    SouthEast = 3,
    South = 4,
    SouthWest = 5,
    West = 6,
    NorthWest = 7,
}

impl Direction {
    pub const fn flip(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::NorthEast => Self::SouthWest,
            Self::East => Self::West,
            Self::SouthEast => Self::NorthWest,
            Self::South => Self::North,
            Self::SouthWest => Self::NorthEast,
            Self::West => Self::East,
            Self::NorthWest => Self::SouthEast,
        }
    }

    /// Rotate the provided vector to fit the direction.
    /// The vector is assumed to be in the north direction.
    pub fn rotate_vector(self, vector: Vector) -> Vector {
        let (x_fac, y_fac, swap) = match self {
            Self::North => (1.0, 1.0, false),
            Self::NorthEast => todo!(),
            Self::East => (-1.0, 1.0, true),
            Self::SouthEast => todo!(),
            Self::South => (1.0, -1.0, false),
            Self::SouthWest => todo!(),
            Self::West => (1.0, -1.0, true),
            Self::NorthWest => todo!(),
        };

        let (x, y) = if swap {
            (vector.1, vector.0)
        } else {
            (vector.0, vector.1)
        };

        (x * x_fac, y * y_fac)
    }

    pub const fn is_straight(&self, other: &Self) -> bool {
        match self {
            Self::NorthEast | Self::SouthWest => matches!(other, Self::NorthEast | Self::SouthWest),
            Self::NorthWest | Self::SouthEast => matches!(other, Self::NorthWest | Self::SouthEast),
            Self::North | Self::South => matches!(other, Self::North | Self::South),
            Self::East | Self::West => matches!(other, Self::East | Self::West),
        }
    }

    pub const fn is_right_angle(&self, other: &Self) -> bool {
        match self {
            Self::NorthEast | Self::SouthWest => matches!(other, Self::NorthWest | Self::SouthEast),
            Self::NorthWest | Self::SouthEast => matches!(other, Self::NorthEast | Self::SouthWest),
            Self::North | Self::South => matches!(other, Self::East | Self::West),
            Self::East | Self::West => matches!(other, Self::North | Self::South),
        }
    }

    pub const fn to_orientation(&self) -> RealOrientation {
        match self {
            Self::North => 0.0,
            Self::NorthEast => 0.125,
            Self::East => 0.25,
            Self::SouthEast => 0.375,
            Self::South => 0.5,
            Self::SouthWest => 0.625,
            Self::West => 0.75,
            Self::NorthWest => 0.875,
        }
    }

    pub fn is_default(other: &Self) -> bool {
        other == &Self::default()
    }
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::North,
            1 => Self::NorthEast,
            2 => Self::East,
            3 => Self::SouthEast,
            4 => Self::South,
            5 => Self::SouthWest,
            6 => Self::West,
            7 => Self::NorthWest,
            _ => panic!("Invalid direction value: {value}"),
        }
    }
}

/// [`Types/DamageTypeID`](https://lua-api.factorio.com/latest/types/DamageTypeID.html)
pub type DamageTypeID = String;

/// Single element of [`Types/Resistances`](https://lua-api.factorio.com/latest/types/Resistances.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct Resistance {
    #[serde(rename = "type")]
    pub type_: DamageTypeID,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub decrease: f64,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub percent: f64,
}

/// [`Types/Resistances`](https://lua-api.factorio.com/latest/types/Resistances.html)
pub type Resistances = EmptyArrayFix<Resistance>;

/// [`Types/RadiusVisualisationSpecification`](https://lua-api.factorio.com/latest/types/RadiusVisualisationSpecification.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RadiusVisualisationSpecification {
    pub sprite: Option<Sprite>,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub distance: f64,

    pub offset: Option<Vector>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_in_cursor: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_on_selection: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LightDefinitionType {
    #[default]
    Basic,
    Oriented,
}

/// [`Types/LightDefinition`](https://lua-api.factorio.com/latest/types/LightDefinition.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct LightDefinitionData {
    // TODO: skip serializing if is default
    #[serde(default, rename = "type")]
    pub type_: LightDefinitionType,

    pub picture: Option<Sprite>,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub rotation_shift: RealOrientation,

    pub intensity: f64,
    pub size: f64,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub source_orientation_offset: RealOrientation,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub add_perspective: bool,

    pub shift: Option<Vector>,
    pub color: Option<Color>,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub minimum_darkness: f64,
}

/// [`Types/LightDefinition`](https://lua-api.factorio.com/latest/types/LightDefinition.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LightDefinition {
    Struct(LightDefinitionData),
    Array(Vec<LightDefinitionData>),
}

/// [`Types/CircuitConnectorSprites`](https://lua-api.factorio.com/latest/types/CircuitConnectorSprites.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitConnectorSprites {
    pub led_red: Sprite,
    pub led_green: Sprite,
    pub led_blue: Sprite,
    pub led_light: LightDefinition,

    pub connector_main: Option<Sprite>,
    pub connector_shadow: Option<Sprite>,

    pub wire_pins: Option<Sprite>,
    pub wire_pins_shadow: Option<Sprite>,

    pub led_blue_off: Option<Sprite>,
    pub led_blue_light_offset: Option<Vector>,
    pub red_green_led_light_offset: Option<Vector>,
}

/// [`Types/BoxSpecification`](https://lua-api.factorio.com/latest/types/BoxSpecification.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct BoxSpecification {
    pub sprite: Sprite,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_whole_box: bool,

    pub side_length: Option<f64>,

    pub side_height: Option<f64>,

    pub max_side_length: Option<f64>,
}

/// [`Types/BeamAnimationSet`](https://lua-api.factorio.com/latest/types/BeamAnimationSet.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct BeamAnimationSet {
    pub start: Option<Animation>,
    pub ending: Option<Animation>,
    pub head: Option<Animation>,
    pub tail: Option<Animation>,
    pub body: Option<AnimationVariations>,
}

/// [`Types/ModuleTint`](https://lua-api.factorio.com/latest/types/ModuleTint.html)
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModuleTint {
    #[default]
    None,
    Primary,
    Secondary,
    Tertiary,
    Quaternary,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModuleTintMode {
    #[default]
    SingleModule,
    Mix,
}

/// [`Types/BeaconModuleVisualization`](https://lua-api.factorio.com/latest/types/BeaconModuleVisualization.html)
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BeaconModuleVisualization {
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_empty_slot: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub draw_as_light: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_as_sprite: bool,

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_i8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub secondary_draw_order: i8,

    // TODO: skip serializing if is default
    #[serde(default)]
    pub apply_module_tint: ModuleTint,

    pub render_layer: Option<RenderLayer>,
    pub pictures: Option<SpriteVariations>,
}

/// [`Types/BeaconModuleVisualizations`](https://lua-api.factorio.com/latest/types/BeaconModuleVisualizations.html)
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BeaconModuleVisualizations {
    pub art_style: String,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub use_for_empty_slots: bool,

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_i32",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub tier_offset: i32,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub slots: Vec<Vec<BeaconModuleVisualization>>,
}

/// [`Types/BeaconGraphicsSet`](https://lua-api.factorio.com/latest/types/BeaconGraphicsSet.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct BeaconGraphicsSet {
    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_animation_when_idle: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub draw_light_when_idle: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub random_animation_offset: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub module_icons_suppressed: bool,

    pub base_layer: Option<RenderLayer>,
    pub animation_layer: Option<RenderLayer>,
    pub top_layer: Option<RenderLayer>,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub animation_progress: f64,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub min_animation_progress: f64,

    #[serde(
        default = "helper::f64_1000",
        skip_serializing_if = "helper::is_1000_f64"
    )]
    pub max_animation_progress: f64,

    // TODO: skip serializing if is default
    #[serde(default)]
    pub apply_module_tint: ModuleTint,

    // TODO: skip serializing if is default
    #[serde(default)]
    pub apply_module_tint_to_light: ModuleTint,

    pub no_modules_tint: Option<Color>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub animation_list: Vec<AnimationElement>,

    pub light: Option<LightDefinition>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub module_visualisations: Vec<BeaconModuleVisualizations>,

    /// TODO: skip serializing if is default
    #[serde(default)]
    pub module_tint_mode: ModuleTintMode,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BeaconGraphicsSetRenderOpts {
    pub runtime_tint: Option<Color>,
}

impl From<&BeaconGraphicsSetRenderOpts> for AnimationRenderOpts {
    fn from(value: &BeaconGraphicsSetRenderOpts) -> Self {
        Self {
            progress: 0.0,
            runtime_tint: value.runtime_tint,
        }
    }
}

impl RenderableGraphics for BeaconGraphicsSet {
    type RenderOpts = BeaconGraphicsSetRenderOpts;

    fn render(
        &self,
        factorio_dir: &str,
        used_mods: &HashMap<&str, &str>,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        // TODO: render module visualisations
        merge_layers(
            &self.animation_list,
            factorio_dir,
            used_mods,
            image_cache,
            &opts.into(),
        )
    }
}

/// [`Types/PumpConnectorGraphicsAnimation`](https://lua-api.factorio.com/latest/types/PumpConnectorGraphicsAnimation.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct PumpConnectorGraphicsAnimation {
    pub standup_base: Option<Animation>,
    pub standup_top: Option<Animation>,
    pub standup_shadow: Option<Animation>,
    pub connector: Option<Animation>,
    pub connector_shadow: Option<Animation>,
}

/// [`Types/PumpConnectorGraphics`](https://lua-api.factorio.com/latest/types/PumpConnectorGraphics.html)
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Serialize, Deserialize)]
pub struct PumpConnectorGraphics {
    pub north: Vec<PumpConnectorGraphicsAnimation>,
    pub east: Vec<PumpConnectorGraphicsAnimation>,
    pub south: Vec<PumpConnectorGraphicsAnimation>,
    pub west: Vec<PumpConnectorGraphicsAnimation>,
}

/// [`Types/CharacterArmorAnimation`](https://lua-api.factorio.com/latest/types/CharacterArmorAnimation.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterArmorAnimation {
    pub idle: RotatedAnimation,
    pub idle_with_gun: RotatedAnimation,
    pub running: RotatedAnimation,
    pub running_with_gun: RotatedAnimation,
    pub mining_with_tool: RotatedAnimation,
    pub flipped_shadow_running_with_gun: Option<RotatedAnimation>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub armors: Vec<String>,
}

/// [`Types/TransportBeltAnimationSet`](https://lua-api.factorio.com/latest/types/TransportBeltAnimationSet.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct TransportBeltAnimationSet {
    pub animation_set: RotatedAnimation,

    #[serde(
        default = "helper::u8_1",
        skip_serializing_if = "helper::is_1_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub east_index: u8,
    #[serde(
        default = "helper::u8_2",
        skip_serializing_if = "helper::is_2_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub west_index: u8,
    #[serde(
        default = "helper::u8_3",
        skip_serializing_if = "helper::is_3_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub north_index: u8,
    #[serde(
        default = "helper::u8_4",
        skip_serializing_if = "helper::is_4_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub south_index: u8,

    #[serde(
        default = "helper::u8_13",
        skip_serializing_if = "helper::is_13_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub starting_south_index: u8,
    #[serde(
        default = "helper::u8_14",
        skip_serializing_if = "helper::is_14_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub ending_south_index: u8,
    #[serde(
        default = "helper::u8_15",
        skip_serializing_if = "helper::is_15_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub starting_west_index: u8,
    #[serde(
        default = "helper::u8_16",
        skip_serializing_if = "helper::is_16_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub ending_west_index: u8,
    #[serde(
        default = "helper::u8_17",
        skip_serializing_if = "helper::is_17_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub starting_north_index: u8,
    #[serde(
        default = "helper::u8_18",
        skip_serializing_if = "helper::is_18_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub ending_north_index: u8,
    #[serde(
        default = "helper::u8_19",
        skip_serializing_if = "helper::is_19_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub starting_east_index: u8,
    #[serde(
        default = "helper::u8_20",
        skip_serializing_if = "helper::is_20_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub ending_east_index: u8,

    pub ending_patch: Option<Sprite4Way>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub ends_with_stopper: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TransportBeltAnimationSetRenderOpts {
    pub direction: Direction,
    pub connections: Option<ConnectedDirections>,

    pub runtime_tint: Option<Color>,

    pub index_override: Option<u8>,
}

impl From<&TransportBeltAnimationSetRenderOpts> for RotatedAnimationRenderOpts {
    fn from(value: &TransportBeltAnimationSetRenderOpts) -> Self {
        Self {
            progress: 0.0,
            runtime_tint: value.runtime_tint,
            orientation: 0.0,
            override_index: value.index_override,
        }
    }
}

impl RenderableGraphics for TransportBeltAnimationSet {
    type RenderOpts = TransportBeltAnimationSetRenderOpts;

    fn render(
        &self,
        factorio_dir: &str,
        used_mods: &HashMap<&str, &str>,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        // -1 because the index is 1-based. Lua stuff :)
        let index = match opts.direction {
            Direction::North => self.north_index - 1,
            Direction::East => self.east_index - 1,
            Direction::South => self.south_index - 1,
            Direction::West => self.west_index - 1,
            _ => unreachable!("Belts only support cardinal directions"),
        };

        let index_options = &Self::RenderOpts {
            index_override: Some(index),
            ..*opts
        };

        self.animation_set
            .render(factorio_dir, used_mods, image_cache, &index_options.into())
    }
}

/// [`Types/TransportBeltAnimationSetWithCorners`](https://lua-api.factorio.com/latest/types/TransportBeltAnimationSetWithCorners.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct TransportBeltAnimationSetWithCorners {
    #[serde(
        default = "helper::u8_5",
        skip_serializing_if = "helper::is_5_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub east_to_north_index: u8,
    #[serde(
        default = "helper::u8_6",
        skip_serializing_if = "helper::is_6_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub north_to_east_index: u8,
    #[serde(
        default = "helper::u8_7",
        skip_serializing_if = "helper::is_7_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub west_to_north_index: u8,
    #[serde(
        default = "helper::u8_8",
        skip_serializing_if = "helper::is_8_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub north_to_west_index: u8,
    #[serde(
        default = "helper::u8_9",
        skip_serializing_if = "helper::is_9_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub south_to_east_index: u8,
    #[serde(
        default = "helper::u8_10",
        skip_serializing_if = "helper::is_10_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub east_to_south_index: u8,
    #[serde(
        default = "helper::u8_11",
        skip_serializing_if = "helper::is_11_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub south_to_west_index: u8,
    #[serde(
        default = "helper::u8_12",
        skip_serializing_if = "helper::is_12_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub west_to_south_index: u8,

    #[serde(flatten)]
    pub animation_set: TransportBeltAnimationSet,
}

impl RenderableGraphics for TransportBeltAnimationSetWithCorners {
    type RenderOpts = TransportBeltAnimationSetRenderOpts;

    fn render(
        &self,
        factorio_dir: &str,
        used_mods: &HashMap<&str, &str>,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        let connections = opts.connections.unwrap_or_default();
        let index = match opts.direction {
            Direction::North => match connections {
                ConnectedDirections::Left | ConnectedDirections::UpLeft => self.west_to_north_index,
                ConnectedDirections::Right | ConnectedDirections::UpRight => {
                    self.east_to_north_index
                }
                _ => self.animation_set.north_index,
            },
            Direction::South => match connections {
                ConnectedDirections::Left | ConnectedDirections::DownLeft => {
                    self.west_to_south_index
                }
                ConnectedDirections::Right | ConnectedDirections::DownRight => {
                    self.east_to_south_index
                }
                _ => self.animation_set.south_index,
            },
            Direction::East => match connections {
                ConnectedDirections::Up | ConnectedDirections::UpRight => self.north_to_east_index,
                ConnectedDirections::Down | ConnectedDirections::DownRight => {
                    self.south_to_east_index
                }
                _ => self.animation_set.east_index,
            },
            Direction::West => match connections {
                ConnectedDirections::Up | ConnectedDirections::UpLeft => self.north_to_west_index,
                ConnectedDirections::Down | ConnectedDirections::DownLeft => {
                    self.south_to_west_index
                }
                _ => self.animation_set.west_index,
            },
            _ => unreachable!("Belts only support cardinal directions"),
        } - 1;

        let index_options = &Self::RenderOpts {
            index_override: Some(index),
            ..*opts
        };

        self.animation_set.animation_set.render(
            factorio_dir,
            used_mods,
            image_cache,
            &index_options.into(),
        )
    }
}

/// [`Types/TransportBeltConnectorFrame`](https://lua-api.factorio.com/latest/types/TransportBeltConnectorFrame.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct TransportBeltConnectorFrame {
    pub frame_main: AnimationVariations,
    pub frame_shadow: AnimationVariations,
    pub frame_main_scanner: Animation,
    pub frame_main_scanner_movement_speed: f64, // docs specify single precision float
    pub frame_main_scanner_horizontal_start_shift: Vector,
    pub frame_main_scanner_horizontal_end_shift: Vector,
    pub frame_main_scanner_horizontal_y_scale: f64, // docs specify single precision float
    pub frame_main_scanner_horizontal_rotation: RealOrientation,
    pub frame_main_scanner_vertical_start_shift: Vector,
    pub frame_main_scanner_vertical_end_shift: Vector,
    pub frame_main_scanner_vertical_rotation: RealOrientation,
    pub frame_main_scanner_cross_horizontal_start_shift: Vector,
    pub frame_main_scanner_cross_horizontal_end_shift: Vector,
    pub frame_main_scanner_cross_horizontal_y_scale: f64, // docs specify single precision float
    pub frame_main_scanner_cross_horizontal_rotation: RealOrientation,
    pub frame_main_scanner_cross_vertical_start_shift: Vector,
    pub frame_main_scanner_cross_vertical_end_shift: Vector,
    pub frame_main_scanner_cross_vertical_y_scale: f64, // docs specify single precision float
    pub frame_main_scanner_cross_vertical_rotation: RealOrientation,
    pub frame_main_scanner_nw_ne: Animation,
    pub frame_main_scanner_sw_se: Animation,
    pub frame_back_patch: Option<SpriteVariations>,
    pub frame_front_patch: Option<SpriteVariations>,
}

/// [`Types/WorkingVisualisation`](https://lua-api.factorio.com/latest/types/WorkingVisualisation.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingVisualisation {
    // TODO: get the default for this
    pub render_layer: Option<RenderLayer>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub fadeout: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub synced_fadeout: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub constant_speed: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub always_draw: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub animated_shift: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub align_to_waypoint: bool,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "helper::truncating_opt_deserializer"
    )]
    pub secondary_draw_order: Option<i8>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_as_sprite: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub draw_as_light: bool,
    pub light: Option<LightDefinition>,
    pub effect: Option<WorkingVisualisationEffect>,
    pub apply_recipe_tint: Option<WorkingVisualisationRecipeTint>,
    pub apply_tint: Option<WorkingVisualisationTint>,

    #[serde(flatten)]
    pub animation: Option<WorkingVisualisationAnimation>,

    pub north_position: Option<Vector>,
    pub west_position: Option<Vector>,
    pub south_position: Option<Vector>,
    pub east_position: Option<Vector>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct WorkingVisualisationRenderOpts {
    pub progress: f64,
    pub runtime_tint: Option<Color>,
    pub direction: Direction,
}

impl From<&WorkingVisualisationRenderOpts> for AnimationRenderOpts {
    fn from(value: &WorkingVisualisationRenderOpts) -> Self {
        Self {
            progress: value.progress,
            runtime_tint: value.runtime_tint,
        }
    }
}

impl From<&MiningDrillGraphicsRenderOpts> for WorkingVisualisationRenderOpts {
    fn from(value: &MiningDrillGraphicsRenderOpts) -> Self {
        Self {
            progress: 0.0,
            runtime_tint: value.runtime_tint,
            direction: value.direction,
        }
    }
}

impl RenderableGraphics for WorkingVisualisation {
    type RenderOpts = WorkingVisualisationRenderOpts;

    fn render(
        &self,
        factorio_dir: &str,
        used_mods: &HashMap<&str, &str>,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        if self.draw_as_light {
            return None;
        }

        self.animation
            .as_ref()?
            .render(factorio_dir, used_mods, image_cache, opts)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkingVisualisationEffect {
    None,
    Flicker,
    UraniumGlow,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WorkingVisualisationRecipeTint {
    None,
    Primary,
    Secondary,
    Tertiary,
    Quaternary,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkingVisualisationTint {
    None,
    Status,
    ResourceColor,
    InputFluidBaseColor,
    InputFluidFlowColor,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkingVisualisationAnimation {
    Single {
        animation: Animation,
    },
    Cardinal {
        north_animation: Option<Animation>,
        east_animation: Option<Animation>,
        south_animation: Option<Animation>,
        west_animation: Option<Animation>,
    },
}

impl RenderableGraphics for WorkingVisualisationAnimation {
    type RenderOpts = WorkingVisualisationRenderOpts;

    fn render(
        &self,
        factorio_dir: &str,
        used_mods: &HashMap<&str, &str>,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        match self {
            WorkingVisualisationAnimation::Single { animation } => {
                animation.render(factorio_dir, used_mods, image_cache, &opts.into())
            }
            WorkingVisualisationAnimation::Cardinal {
                north_animation,
                east_animation,
                south_animation,
                west_animation,
            } => match opts.direction {
                Direction::North => north_animation.as_ref(),
                Direction::East => east_animation.as_ref(),
                Direction::South => south_animation.as_ref(),
                Direction::West => west_animation.as_ref(),
                _ => return None,
            }
            .and_then(|a| a.render(factorio_dir, used_mods, image_cache, &opts.into())),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum GuiMode {
    All,
    None,
    Admins,
}

impl GuiMode {
    pub const fn all() -> Self {
        Self::All
    }

    pub const fn none() -> Self {
        Self::None
    }

    pub const fn admins() -> Self {
        Self::Admins
    }

    pub const fn is_all(value: &Self) -> bool {
        matches!(value, Self::All)
    }

    pub const fn is_none(value: &Self) -> bool {
        matches!(value, Self::None)
    }

    pub const fn is_admins(value: &Self) -> bool {
        matches!(value, Self::Admins)
    }
}

/// [`Types/HeatConnection`](https://lua-api.factorio.com/latest/types/HeatConnection.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct HeatConnection {
    pub position: MapPosition,
    pub direction: Direction,
}

/// [`Types/HeatBuffer`](https://lua-api.factorio.com/latest/types/HeatBuffer.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct HeatBuffer {
    pub max_temperature: f64,
    pub specific_heat: Energy,
    pub max_transfer: Energy,

    #[serde(default = "helper::f64_15", skip_serializing_if = "helper::is_15_f64")]
    pub default_temperature: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub min_temperature_gradient: f64,

    #[serde(default = "helper::f64_15", skip_serializing_if = "helper::is_15_f64")]
    pub min_working_temperature: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub minimum_glow_temperature: f64,

    pub pipe_covers: Option<Sprite4Way>,
    pub heat_pipe_covers: Option<Sprite4Way>,
    pub heat_picture: Option<Sprite4Way>,
    pub heat_glow: Option<Sprite4Way>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub connections: Vec<HeatConnection>,
}

/// [`Types/ConnectableEntityGraphics`](https://lua-api.factorio.com/latest/types/ConnectableEntityGraphics.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectableEntityGraphics {
    pub single: SpriteVariations,
    pub straight_vertical: SpriteVariations,
    pub straight_horizontal: SpriteVariations,

    pub corner_left_up: SpriteVariations,
    pub corner_right_up: SpriteVariations,
    pub corner_left_down: SpriteVariations,
    pub corner_right_down: SpriteVariations,

    pub t_up: SpriteVariations,
    pub t_down: SpriteVariations,
    pub t_left: SpriteVariations,
    pub t_right: SpriteVariations,

    pub ending_up: SpriteVariations,
    pub ending_down: SpriteVariations,
    pub ending_left: SpriteVariations,
    pub ending_right: SpriteVariations,

    pub cross: SpriteVariations,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ConnectedDirections {
    #[default]
    None,

    Up,
    Down,
    Left,
    Right,

    UpDown,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    LeftRight,

    UpDownLeft,
    UpDownRight,
    UpLeftRight,
    DownLeftRight,

    All,
}

impl ConnectedDirections {
    #[must_use]
    pub const fn from_directions(up: bool, down: bool, left: bool, right: bool) -> Self {
        match (up, down, left, right) {
            (false, false, false, false) => Self::None,
            (true, false, false, false) => Self::Up,
            (false, true, false, false) => Self::Down,
            (false, false, true, false) => Self::Left,
            (false, false, false, true) => Self::Right,
            (true, true, false, false) => Self::UpDown,
            (true, false, true, false) => Self::UpLeft,
            (true, false, false, true) => Self::UpRight,
            (false, true, true, false) => Self::DownLeft,
            (false, true, false, true) => Self::DownRight,
            (false, false, true, true) => Self::LeftRight,
            (true, true, true, false) => Self::UpDownLeft,
            (true, true, false, true) => Self::UpDownRight,
            (true, false, true, true) => Self::UpLeftRight,
            (false, true, true, true) => Self::DownLeftRight,
            (true, true, true, true) => Self::All,
        }
    }
}

impl ConnectableEntityGraphics {
    pub fn get(&self, connections: ConnectedDirections) -> &SpriteVariations {
        match connections {
            ConnectedDirections::None => &self.single,
            ConnectedDirections::Up => &self.ending_up,
            ConnectedDirections::Down => &self.ending_down,
            ConnectedDirections::Left => &self.ending_left,
            ConnectedDirections::Right => &self.ending_right,
            ConnectedDirections::UpDown => &self.straight_vertical,
            ConnectedDirections::UpLeft => &self.corner_left_up,
            ConnectedDirections::UpRight => &self.corner_right_up,
            ConnectedDirections::DownLeft => &self.corner_left_down,
            ConnectedDirections::DownRight => &self.corner_right_down,
            ConnectedDirections::LeftRight => &self.straight_horizontal,
            ConnectedDirections::UpDownLeft => &self.t_left,
            ConnectedDirections::UpDownRight => &self.t_right,
            ConnectedDirections::UpLeftRight => &self.t_up,
            ConnectedDirections::DownLeftRight => &self.t_down,
            ConnectedDirections::All => &self.cross,
        }
    }
}

/// [`Types/ForceCondition`](https://lua-api.factorio.com/latest/types/ForceCondition.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ForceCondition {
    All,
    Ally,
    Same,
    Enemy,
    Friend,
    NotSame,
    NotFriend,
}

impl ForceCondition {
    pub const fn all() -> Self {
        Self::All
    }

    pub const fn ally() -> Self {
        Self::Ally
    }

    pub const fn same() -> Self {
        Self::Same
    }

    pub const fn enemy() -> Self {
        Self::Enemy
    }

    pub const fn friend() -> Self {
        Self::Friend
    }

    pub const fn not_same() -> Self {
        Self::NotSame
    }

    pub const fn not_friend() -> Self {
        Self::NotFriend
    }

    pub const fn is_all(value: &Self) -> bool {
        matches!(value, Self::All)
    }

    pub const fn is_ally(value: &Self) -> bool {
        matches!(value, Self::Ally)
    }

    pub const fn is_same(value: &Self) -> bool {
        matches!(value, Self::Same)
    }

    pub const fn is_enemy(value: &Self) -> bool {
        matches!(value, Self::Enemy)
    }

    pub const fn is_friend(value: &Self) -> bool {
        matches!(value, Self::Friend)
    }

    pub const fn is_not_same(value: &Self) -> bool {
        matches!(value, Self::NotSame)
    }

    pub const fn is_not_friend(value: &Self) -> bool {
        matches!(value, Self::NotFriend)
    }
}

/// [`Types/MiningDrillGraphicsSet`](https://lua-api.factorio.com/latest/types/MiningDrillGraphicsSet.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct MiningDrillGraphicsSet {
    pub animation: Option<Animation4Way>,
    pub idle_animation: Option<Animation4Way>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub always_draw_idle_animation: bool,

    pub default_recipe_tint: Option<Color>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub working_visualisations: Vec<WorkingVisualisation>,

    pub shift_animation_waypoints: Option<ShiftAnimationWaypoints>,

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_u16",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub shift_animation_waypoint_stop_duration: u16,

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_u16",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub shift_animation_transition_duration: u16,

    pub status_colors: Option<StatusColors>,

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_u16",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub drilling_vertical_movement_duration: u16,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub animation_progress: f64, // specified as single precision in docs

    #[serde(
        default = "helper::f64_1000",
        skip_serializing_if = "helper::is_1000_f64"
    )]
    pub max_animation_progress: f64, // specified as single precision in docs

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub min_animation_progress: f64, // specified as single precision in docs

    pub circuit_connector_layer: CircuitConnectorLayer,
    pub circuit_connector_secondary_draw_order: CircuitConnectorSecondaryDrawOrder,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MiningDrillGraphicsRenderOpts {
    pub direction: Direction,
    pub runtime_tint: Option<Color>,
}

impl From<&MiningDrillGraphicsRenderOpts> for Animation4WayRenderOpts {
    fn from(value: &MiningDrillGraphicsRenderOpts) -> Self {
        Self {
            direction: value.direction,
            progress: 0.0,
            runtime_tint: value.runtime_tint,
        }
    }
}

impl RenderableGraphics for MiningDrillGraphicsSet {
    type RenderOpts = MiningDrillGraphicsRenderOpts;

    fn render(
        &self,
        factorio_dir: &str,
        used_mods: &HashMap<&str, &str>,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        // TODO: fix for electric drills
        let mut renders = vec![self
            .idle_animation
            .as_ref()
            .or(self.animation.as_ref())
            .and_then(|a| a.render(factorio_dir, used_mods, image_cache, &opts.into()))];

        renders.extend(
            self.working_visualisations
                .iter()
                .map(|wv| wv.render(factorio_dir, used_mods, image_cache, &opts.into())),
        );

        merge_renders(&renders)
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CircuitConnectorLayer {
    Single(Option<RenderLayer>),
    Directional {
        // TODO: defaults
        north: Option<RenderLayer>,
        east: Option<RenderLayer>,
        south: Option<RenderLayer>,
        west: Option<RenderLayer>,
    },
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CircuitConnectorSecondaryDrawOrder {
    Single(
        #[serde(
            default = "helper::i8_100",
            skip_serializing_if = "helper::is_100_i8",
            deserialize_with = "helper::truncating_deserializer"
        )]
        i8,
    ),
    Directional {
        #[serde(
            default = "helper::i8_100",
            skip_serializing_if = "helper::is_100_i8",
            deserialize_with = "helper::truncating_deserializer"
        )]
        north: i8,

        #[serde(
            default = "helper::i8_100",
            skip_serializing_if = "helper::is_100_i8",
            deserialize_with = "helper::truncating_deserializer"
        )]
        east: i8,

        #[serde(
            default = "helper::i8_100",
            skip_serializing_if = "helper::is_100_i8",
            deserialize_with = "helper::truncating_deserializer"
        )]
        south: i8,

        #[serde(
            default = "helper::i8_100",
            skip_serializing_if = "helper::is_100_i8",
            deserialize_with = "helper::truncating_deserializer"
        )]
        west: i8,
    },
}

/// [`Types/RailPictureSet`](https://lua-api.factorio.com/latest/types/RailPictureSet.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct RailPictureSet {
    pub straight_rail_horizontal: RailPieceLayers,
    pub straight_rail_vertical: RailPieceLayers,
    pub straight_rail_diagonal_left_top: RailPieceLayers,
    pub straight_rail_diagonal_right_top: RailPieceLayers,
    pub straight_rail_diagonal_right_bottom: RailPieceLayers,
    pub straight_rail_diagonal_left_bottom: RailPieceLayers,
    pub curved_rail_vertical_left_top: RailPieceLayers,
    pub curved_rail_vertical_right_top: RailPieceLayers,
    pub curved_rail_vertical_right_bottom: RailPieceLayers,
    pub curved_rail_vertical_left_bottom: RailPieceLayers,
    pub curved_rail_horizontal_left_top: RailPieceLayers,
    pub curved_rail_horizontal_right_top: RailPieceLayers,
    pub curved_rail_horizontal_right_bottom: RailPieceLayers,
    pub curved_rail_horizontal_left_bottom: RailPieceLayers,
    pub rail_endings: Sprite8Way,
}

/// [`Types/RailPieceLayers`](https://lua-api.factorio.com/latest/types/RailPieceLayers.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RailPieceLayers {
    pub metals: SpriteVariations,
    pub backplates: SpriteVariations,
    pub ties: SpriteVariations,
    pub stone_path: SpriteVariations,

    pub stone_path_background: Option<SpriteVariations>,
    pub segment_visualisation_middle: Option<SpriteVariations>,
    pub segment_visualisation_ending_front: Option<SpriteVariations>,
    pub segment_visualisation_ending_back: Option<SpriteVariations>,
    pub segment_visualisation_continuing_front: Option<SpriteVariations>,
    pub segment_visualisation_continuing_back: Option<SpriteVariations>,
}

/// [`Types/TrainStopLight`](https://lua-api.factorio.com/latest/types/TrainStopLight.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct TrainStopLight {
    pub picture: Sprite4Way,
    pub red_picture: Sprite4Way,
    pub light: LightDefinition,
}

// Comparator variants
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum Comparator {
    #[serde(rename = "<")]
    Less,
    #[serde(rename = "≤", alias = "<=")]
    LessOrEqual,
    #[serde(rename = ">")]
    Greater,
    #[serde(rename = "≥", alias = ">=")]
    GreaterOrEqual,
    #[serde(rename = "=")]
    Equal,
    #[serde(rename = "≠", alias = "!=")]
    NotEqual,
}

// https://lua-api.factorio.com/latest/concepts.html#ArithmeticCombinatorParameters
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum ArithmeticOperation {
    #[serde(rename = "*")]
    Multiply,

    #[serde(rename = "/")]
    Divide,

    #[serde(rename = "+")]
    Add,

    #[serde(rename = "-")]
    Subtract,

    #[serde(rename = "%")]
    Modulo,

    #[serde(rename = "^")]
    Power,

    #[serde(rename = "<<")]
    LeftShift,

    #[serde(rename = ">>")]
    RightShift,

    #[serde(rename = "AND")]
    BitwiseAnd,

    #[serde(rename = "OR")]
    BitwiseOr,

    #[serde(rename = "XOR")]
    BitwiseXor,
}
