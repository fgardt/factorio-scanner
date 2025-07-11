#![allow(
    unused_variables,
    clippy::wildcard_imports,
    clippy::struct_excessive_bools,
    clippy::module_name_repetitions
)]

use std::{collections::HashMap, fmt, hash::Hash};

use konst::{
    iter::collect_const, primitive::parse_u16, result::unwrap_ctx, string::split as konst_split,
};
use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;
use tracing::warn;

use mod_util::{mod_info::Version, UsedMods};

#[must_use]
pub const fn targeted_engine_version() -> Version {
    const V: [&str; 3] = collect_const!(&str => konst_split(env!("CARGO_PKG_VERSION_PRE"), '.'));
    Version::new(
        unwrap_ctx!(parse_u16(V[0])),
        unwrap_ctx!(parse_u16(V[1])),
        unwrap_ctx!(parse_u16(V[2])),
    )
}

mod empty_array_fix;
mod energy;
mod fluid_box;
mod graphics;
mod icon;
mod ids;
mod item;
mod module;
mod wire;

pub use empty_array_fix::*;
pub use energy::*;
pub use fluid_box::*;
pub use graphics::*;
pub use icon::*;
pub use ids::*;
pub use item::*;
pub use module::*;
pub use wire::*;

/// Generic type for Factorio's commonly used pattern of
/// allowing either a single direct value or an array of values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SingleOrArray<T> {
    Single(T),
    Array(FactorioArray<T>),
}

/// [`Types/AmmoType`](https://lua-api.factorio.com/latest/types/AmmoType.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct AmmoType {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub clamp_position: bool,

    pub energy_consumption: Option<Energy>,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub range_modifier: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub cooldown_modifier: f64,

    #[serde(default = "helper::f32_1", skip_serializing_if = "helper::is_1_f32")]
    pub consumption_modifier: f32,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub target_type: AmmoTypeTargetType,

    pub source_type: Option<AmmoSourceType>,
    // not implemented
    // pub action: Option<Trigger>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AmmoTypeTargetType {
    #[default]
    Entity,
    Position,
    Direction,
}

/// [`Types/AmmoSourceType`](https://lua-api.factorio.com/latest/types/AmmoSourceType.html)
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AmmoSourceType {
    #[default]
    Default,
    Player,
    Turret,
    Vehicle,
}

/// [`Types/BaseAttackParameters`](https://lua-api.factorio.com/latest/types/BaseAttackParameters.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct BaseAttackParameters {
    pub range: f32,
    pub cooldown: f32,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub min_range: f32,

    #[serde(default = "helper::f32_1", skip_serializing_if = "helper::is_1_f32")]
    pub turn_range: f32,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub fire_penalty: f32,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub rotate_penalty: f32,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub health_penalty: f32,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub range_mode: BaseAttackParametersRangeMode,

    // default is value of range property
    pub min_attack_distance: Option<f32>,

    #[serde(default = "helper::f32_1", skip_serializing_if = "helper::is_1_f32")]
    pub damage_modifier: f32,

    #[serde(default = "helper::f32_1", skip_serializing_if = "helper::is_1_f32")]
    pub ammo_consumption_modifier: f32,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub cooldown_deviation: f32,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub warmup: u32,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub lead_target_for_projectile_speed: f32,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub lead_target_for_projectile_delay: u32,

    // default is value of cooldown property
    pub movement_slow_down_cool_down: Option<f32>,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub movement_slow_down_factor: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub activation_type: BaseAttackParametersActivationType,

    pub animation: Option<RotatedAnimation>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub use_shooter_direction: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub true_collinear_ejection: bool,
    // not implemented
    // ammo_type, ammo_categories, ammo_category: are these mutually exclusive?
    // sound, cyclic_sound
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BaseAttackParametersRangeMode {
    #[default]
    CenterToCenter,
    BoundingBoxToBoundingBox,
    CenterToBoundingBox,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BaseAttackParametersActivationType {
    #[default]
    Shoot,
    Throw,
    Consume,
    Activate,
}

/// [`Types/AttackParameters`](https://lua-api.factorio.com/latest/types/AttackParameters.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AttackParameters {
    /// [`Types/ProjectileAttackParameters`](https://lua-api.factorio.com/latest/types/ProjectileAttackParameters.html)
    #[serde(rename = "projectile")]
    ProjectileAttackParameters {
        #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
        apply_projection_to_projectile_creation_position: bool,

        #[serde(default, skip_serializing_if = "helper::is_default")]
        projectile_center: Vector,

        #[serde(default, skip_serializing_if = "helper::is_default")]
        projectile_creation_distance: f32,

        #[serde(default, skip_serializing_if = "helper::is_default")]
        projectile_orientation_offset: f32,

        #[serde(flatten)]
        base: BaseAttackParameters,
        // not implemented
        // shell_particle, projectile_creation_parameters
    },

    /// [`Types/BeamAttackParameters`](https://lua-api.factorio.com/latest/types/BeamAttackParameters.html)
    #[serde(rename = "beam")]
    BeamAttackParameters {
        #[serde(default, skip_serializing_if = "helper::is_default")]
        source_direction_count: u32,

        source_offset: Option<Vector>,

        #[serde(flatten)]
        base: BaseAttackParameters,
    },

    /// [`Types/StreamAttackParameters`](https://lua-api.factorio.com/latest/types/StreamAttackParameters.html)
    #[serde(rename = "stream")]
    StreamAttackParameters {
        #[serde(default, skip_serializing_if = "helper::is_default")]
        fluid_consumption: FluidAmount,

        #[serde(default, skip_serializing_if = "helper::is_default")]
        gun_barrel_length: f32,

        gun_center_shift: Option<GunShift4WayUnion>,

        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        fluids: FactorioArray<StreamFluidProperties>,

        #[serde(flatten)]
        base: BaseAttackParameters,
        // not implemented
        // projectile_creation_parameters
    },
}

impl std::ops::Deref for AttackParameters {
    type Target = BaseAttackParameters;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::BeamAttackParameters { base, .. }
            | Self::ProjectileAttackParameters { base, .. }
            | Self::StreamAttackParameters { base, .. } => base,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GunShift4WayUnion {
    Single(Vector),
    Directed {
        north: Vector,
        east: Option<Vector>,
        south: Option<Vector>,
        west: Option<Vector>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamFluidProperties {
    #[serde(rename = "type")]
    pub type_: FluidID,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub damage_modifier: f64,
}

/// [`Types/CapsuleAction`](https://lua-api.factorio.com/latest/types/CapsuleAction.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CapsuleAction {
    /// [`Types/ThrowCapsuleAction`](https://lua-api.factorio.com/latest/types/ThrowCapsuleAction.html)
    #[serde(rename = "throw")]
    ThrowCapsuleAction {
        attack_parameters: AttackParameters,

        #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
        uses_stack: bool,
    },

    /// [`Types/ActivateCapsuleAction`](https://lua-api.factorio.com/latest/types/ActivateCapsuleAction.html)
    #[serde(rename = "equipment-remote")]
    ActivateEquipmentCapsuleAction { equipment: EquipmentID },

    /// [`Types/UseOnSelfCapsuleAction`](https://lua-api.factorio.com/latest/types/UseOnSelfCapsuleAction.html)
    #[serde(rename = "use-on-self")]
    UseOnSelfCapsuleAction {
        attack_parameters: AttackParameters,

        #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
        uses_stack: bool,
    },

    /// [`Types/DestroyCliffsCapsuleAction`](https://lua-api.factorio.com/latest/types/DestroyCliffsCapsuleAction.html)
    #[serde(rename = "destroy-cliffs")]
    DestroyCliffsCapsuleAction {
        attack_parameters: AttackParameters,
        radius: f32,

        #[serde(
            default = "helper::u32_3600",
            skip_serializing_if = "helper::is_3600_u32"
        )]
        timeout: u32,

        #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
        play_sound_on_failure: bool,

        #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
        uses_stack: bool,
    },

    /// [`Types/ArtilleryRemoteCapsuleAction`](https://lua-api.factorio.com/latest/types/ArtilleryRemoteCapsuleAction.html)
    #[serde(rename = "artillery-remote")]
    ArtilleryRemoteCapsuleAction {
        flare: EntityID,

        #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
        play_sound_on_failure: bool,
    },
}

/// [`Types/Color`](https://lua-api.factorio.com/latest/types/Color.html)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Color {
    Struct {
        #[serde(default, skip_serializing_if = "helper::is_default")]
        r: f64,
        #[serde(default, skip_serializing_if = "helper::is_default")]
        g: f64,
        #[serde(default, skip_serializing_if = "helper::is_default")]
        b: f64,
        #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
        a: f64,
    },
    RGB(f64, f64, f64),
    RGBA(f64, f64, f64, f64),
}

impl Color {
    #[must_use]
    pub fn to_rgba(&self) -> [f64; 4] {
        let (r, g, b, a) = match self {
            Self::Struct { r, g, b, a } | Self::RGBA(r, g, b, a) => {
                (Some(*r), Some(*g), Some(*b), Some(*a))
            }
            Self::RGB(r, g, b) => (Some(*r), Some(*g), Some(*b), None::<f64>),
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

    #[must_use]
    pub const fn white() -> Self {
        Self::RGBA(1.0, 1.0, 1.0, 1.0)
    }

    #[must_use]
    pub fn is_white(color: &Self) -> bool {
        let [r, g, b, a] = color.to_rgba();

        (r - 1.0).abs() < f64::EPSILON
            && (g - 1.0).abs() < f64::EPSILON
            && (b - 1.0).abs() < f64::EPSILON
            && (a - 1.0).abs() < f64::EPSILON
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::RGBA(0.0, 0.0, 0.0, 1.0)
    }
}

/// [`Types/SurfaceCondition`](https://lua-api.factorio.com/latest/types/SurfaceCondition.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct SurfaceCondition {
    pub property: SurfacePropertyID,

    #[serde(
        default = "helper::f64_min",
        skip_serializing_if = "helper::is_min_f64"
    )]
    pub min: f64,
    #[serde(
        default = "helper::f64_max",
        skip_serializing_if = "helper::is_max_f64"
    )]
    pub max: f64,
}

/// [`Types/Vector`](https://lua-api.factorio.com/latest/types/Vector.html)
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(untagged)]
pub enum Vector {
    Tuple(f64, f64),
    Struct { x: f64, y: f64 },
}

impl Vector {
    #[must_use]
    pub const fn new(x: f64, y: f64) -> Self {
        Self::Tuple(x, y)
    }

    #[must_use]
    pub const fn x(&self) -> f64 {
        match self {
            Self::Tuple(x, _) | Self::Struct { x, .. } => *x,
        }
    }

    #[must_use]
    pub const fn y(&self) -> f64 {
        match self {
            Self::Tuple(_, y) | Self::Struct { y, .. } => *y,
        }
    }

    #[must_use]
    pub const fn as_tuple(&self) -> (f64, f64) {
        match self {
            Self::Tuple(x, y) | Self::Struct { x, y } => (*x, *y),
        }
    }

    /// Rotate the vector by the given orientation
    #[must_use]
    pub fn rotate(&self, orientation: RealOrientation) -> Self {
        let (x, y) = self.as_tuple();

        let rad = orientation * std::f64::consts::TAU;
        let sin = rad.sin();
        let cos = rad.cos();

        Self::Tuple(x.mul_add(cos, -y * sin), x.mul_add(sin, y * cos))
    }

    #[must_use]
    pub fn flip(&self) -> Self {
        let (x, y) = self.as_tuple();

        Self::Tuple(-x, -y)
    }

    #[must_use]
    pub fn flip_x(&self) -> Self {
        let (x, y) = self.as_tuple();

        Self::Tuple(-x, y)
    }

    #[must_use]
    pub fn flip_y(&self) -> Self {
        let (x, y) = self.as_tuple();

        Self::Tuple(x, -y)
    }

    #[must_use]
    pub fn is_0_vector(value: &Self) -> bool {
        value.x() == 0.0 && value.y() == 0.0
    }

    #[must_use]
    pub fn shorten_by(&self, length: f64) -> Self {
        let (x, y) = self.as_tuple();
        let len = x.hypot(y);

        if len == 0.0 {
            return *self;
        }

        let factor = (len - length) / len;

        Self::Tuple(x * factor, y * factor)
    }
}

impl Default for Vector {
    fn default() -> Self {
        Self::Tuple(Default::default(), Default::default())
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        let (x1, y1) = self.as_tuple();
        let (x2, y2) = other.as_tuple();

        (x1 - x2).abs() < f64::EPSILON && (y1 - y2).abs() < f64::EPSILON
    }
}

impl From<(f64, f64)> for Vector {
    fn from((x, y): (f64, f64)) -> Self {
        Self::Tuple(x, y)
    }
}

impl From<Vector> for (f64, f64) {
    fn from(vector: Vector) -> Self {
        vector.as_tuple()
    }
}

impl From<MapPosition> for Vector {
    fn from(map_position: MapPosition) -> Self {
        let (x, y) = map_position.as_tuple();

        Self::Tuple(x, y)
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (x, y) = self.as_tuple();

        write!(f, "({x}, {y})")
    }
}

impl std::ops::Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (x1, y1) = self.as_tuple();
        let (x2, y2) = rhs.as_tuple();

        Self::Tuple(x1 + x2, y1 + y2)
    }
}

impl std::ops::Add for &Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        let (x1, y1) = self.as_tuple();
        let (x2, y2) = rhs.as_tuple();

        Vector::Tuple(x1 + x2, y1 + y2)
    }
}

impl std::ops::Add<&Self> for Vector {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        let (x1, y1) = self.as_tuple();
        let (x2, y2) = rhs.as_tuple();

        Self::Tuple(x1 + x2, y1 + y2)
    }
}

impl std::ops::Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let (x1, y1) = self.as_tuple();
        let (x2, y2) = rhs.as_tuple();

        Self::Tuple(x1 - x2, y1 - y2)
    }
}

impl std::ops::Sub for &Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        let (x1, y1) = self.as_tuple();
        let (x2, y2) = rhs.as_tuple();

        Vector::Tuple(x1 - x2, y1 - y2)
    }
}

impl std::ops::Sub<&Self> for Vector {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        let (x1, y1) = self.as_tuple();
        let (x2, y2) = rhs.as_tuple();

        Self::Tuple(x1 - x2, y1 - y2)
    }
}

impl std::ops::Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let (x, y) = self.as_tuple();

        Self::Tuple(x * rhs, y * rhs)
    }
}

impl std::ops::Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let (x, y) = self.as_tuple();

        Self::Tuple(x / rhs, y / rhs)
    }
}

impl std::ops::AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        let (x1, y1) = self.as_tuple();
        let (x2, y2) = rhs.as_tuple();

        *self = Self::Tuple(x1 + x2, y1 + y2);
    }
}

impl std::ops::SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        let (x1, y1) = self.as_tuple();
        let (x2, y2) = rhs.as_tuple();

        *self = Self::Tuple(x1 - x2, y1 - y2);
    }
}

impl std::ops::MulAssign<f64> for Vector {
    fn mul_assign(&mut self, rhs: f64) {
        let (x, y) = self.as_tuple();

        *self = Self::Tuple(x * rhs, y * rhs);
    }
}

impl std::ops::DivAssign<f64> for Vector {
    fn div_assign(&mut self, rhs: f64) {
        let (x, y) = self.as_tuple();

        *self = Self::Tuple(x / rhs, y / rhs);
    }
}

/// [`Types/Vector3D`](https://lua-api.factorio.com/latest/types/Vector3D.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Vector3D {
    Struct { x: f64, y: f64, z: f64 },
    Tuple(f64, f64, f64),
}

impl PartialEq for Vector3D {
    fn eq(&self, other: &Self) -> bool {
        let (x1, y1, z1) = match self {
            Self::Tuple(x, y, z) | Self::Struct { x, y, z } => (*x, *y, *z),
        };

        let (x2, y2, z2) = match other {
            Self::Struct { x, y, z } | Self::Tuple(x, y, z) => (*x, *y, *z),
        };

        (x1 - x2).abs() < f64::EPSILON
            && (y1 - y2).abs() < f64::EPSILON
            && (z1 - z2).abs() < f64::EPSILON
    }
}

/// [`Types/FileName`](https://lua-api.factorio.com/latest/types/FileName.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileName(String);

pub type ImageCache = HashMap<String, Option<image::DynamicImage>>;

impl FileName {
    #[must_use]
    pub const fn new(filename: String) -> Self {
        Self(filename)
    }

    pub fn load<'a>(
        &self,
        used_mods: &UsedMods,
        image_cache: &'a mut ImageCache,
    ) -> Option<&'a image::DynamicImage> {
        let filename = &self.0;

        if image_cache.contains_key(filename) {
            return image_cache.get(filename)?.as_ref();
        }

        let re = regex::Regex::new(r"^__([^/\\]+)__").ok()?;
        let mod_name = re.captures(filename)?.get(1)?.as_str();
        let sprite_path = &filename[(2 + mod_name.len() + 2 + 1)..]; // +1 to include the slash to prevent joining to interpret it as a absolute path

        let Some(m) = used_mods.get(mod_name) else {
            warn!("Mod {mod_name} not found");
            return None;
        };

        let file_data = match m.get_file(sprite_path) {
            Ok(d) => d,
            Err(e) => {
                warn!("Error loading {filename}: {e}");
                return None;
            }
        };

        let img = image::load_from_memory_with_format(
            &used_mods.get(mod_name)?.get_file(sprite_path).ok()?,
            image::ImageFormat::Png,
        )
        .ok();

        image_cache.insert(filename.clone(), img);
        image_cache.get(filename)?.as_ref()
    }
}

/// [`Types/LocalisedString`](https://lua-api.factorio.com/latest/types/LocalisedString.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LocalisedString {
    Bool(bool),
    String(String),
    Number(f64),
    Array(FactorioArray<LocalisedString>),
}

/// [`Types/Order`](https://lua-api.factorio.com/latest/types/Order.html)
pub type Order = String;

/// [`Types/RealOrientation`](https://lua-api.factorio.com/latest/types/RealOrientation.html)
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct RealOrientation(f64); // TODO: should be f32

impl RealOrientation {
    #[must_use]
    pub const fn new(orientation: f64) -> Self {
        Self(orientation)
    }

    #[must_use]
    pub fn projected_orientation(&self) -> Self {
        if *self == 0.0 || *self == 0.25 || *self == 0.5 || *self == 0.75 {
            return *self;
        }

        let rad = self.0 * std::f64::consts::TAU;
        let x = rad.cos();
        let y = rad.sin() * std::f64::consts::FRAC_1_SQRT_2;
        let res = y.atan2(x) / std::f64::consts::TAU;

        Self((res + 1.0) % 1.0)
    }
}

impl From<f64> for RealOrientation {
    fn from(f: f64) -> Self {
        Self(f)
    }
}

impl From<RealOrientation> for f64 {
    fn from(orientation: RealOrientation) -> Self {
        *orientation
    }
}

impl PartialEq for RealOrientation {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < f64::EPSILON
    }
}

impl PartialEq<f64> for RealOrientation {
    fn eq(&self, other: &f64) -> bool {
        (self.0 - *other).abs() < f64::EPSILON
    }
}

impl PartialEq<RealOrientation> for f64 {
    fn eq(&self, other: &RealOrientation) -> bool {
        (*self - other.0).abs() < Self::EPSILON
    }
}

impl PartialOrd for RealOrientation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl PartialOrd<f64> for RealOrientation {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<RealOrientation> for f64 {
    fn partial_cmp(&self, other: &RealOrientation) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.0)
    }
}

impl std::ops::Deref for RealOrientation {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Add for RealOrientation {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Add<f64> for RealOrientation {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl std::ops::Add<RealOrientation> for f64 {
    type Output = RealOrientation;

    fn add(self, rhs: RealOrientation) -> Self::Output {
        RealOrientation(self + rhs.0)
    }
}

impl std::ops::AddAssign for RealOrientation {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::ops::AddAssign<f64> for RealOrientation {
    fn add_assign(&mut self, rhs: f64) {
        self.0 += rhs;
    }
}

impl std::ops::AddAssign<RealOrientation> for f64 {
    fn add_assign(&mut self, rhs: RealOrientation) {
        *self += rhs.0;
    }
}

impl std::ops::Sub for RealOrientation {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Sub<f64> for RealOrientation {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl std::ops::Sub<RealOrientation> for f64 {
    type Output = RealOrientation;

    fn sub(self, rhs: RealOrientation) -> Self::Output {
        RealOrientation(self - rhs.0)
    }
}

impl std::ops::SubAssign for RealOrientation {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl std::ops::SubAssign<f64> for RealOrientation {
    fn sub_assign(&mut self, rhs: f64) {
        self.0 -= rhs;
    }
}

impl std::ops::SubAssign<RealOrientation> for f64 {
    fn sub_assign(&mut self, rhs: RealOrientation) {
        *self -= rhs.0;
    }
}

impl std::ops::Mul for RealOrientation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl std::ops::Mul<f64> for RealOrientation {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl std::ops::Mul<RealOrientation> for f64 {
    type Output = RealOrientation;

    fn mul(self, rhs: RealOrientation) -> Self::Output {
        RealOrientation(self * rhs.0)
    }
}

impl std::ops::MulAssign for RealOrientation {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl std::ops::MulAssign<f64> for RealOrientation {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
    }
}

impl std::ops::MulAssign<RealOrientation> for f64 {
    fn mul_assign(&mut self, rhs: RealOrientation) {
        *self *= rhs.0;
    }
}

impl std::ops::Div for RealOrientation {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl std::ops::Div<f64> for RealOrientation {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl std::ops::Div<RealOrientation> for f64 {
    type Output = RealOrientation;

    fn div(self, rhs: RealOrientation) -> Self::Output {
        RealOrientation(self / rhs.0)
    }
}

impl std::ops::DivAssign for RealOrientation {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl std::ops::DivAssign<f64> for RealOrientation {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
    }
}

impl std::ops::DivAssign<RealOrientation> for f64 {
    fn div_assign(&mut self, rhs: RealOrientation) {
        *self /= rhs.0;
    }
}

impl std::ops::Rem for RealOrientation {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0)
    }
}

impl std::ops::Rem<f64> for RealOrientation {
    type Output = Self;

    fn rem(self, rhs: f64) -> Self::Output {
        Self(self.0 % rhs)
    }
}

impl std::ops::Rem<RealOrientation> for f64 {
    type Output = RealOrientation;

    fn rem(self, rhs: RealOrientation) -> Self::Output {
        RealOrientation(self % rhs.0)
    }
}

impl std::ops::RemAssign for RealOrientation {
    fn rem_assign(&mut self, rhs: Self) {
        self.0 %= rhs.0;
    }
}

impl std::ops::RemAssign<f64> for RealOrientation {
    fn rem_assign(&mut self, rhs: f64) {
        self.0 %= rhs;
    }
}

impl std::ops::RemAssign<RealOrientation> for f64 {
    fn rem_assign(&mut self, rhs: RealOrientation) {
        *self %= rhs.0;
    }
}

impl std::ops::Neg for RealOrientation {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SignalIDConnector {
    Virtual { name: VirtualSignalID },
    Item { name: ItemID },
    Fluid { name: FluidID },
}

/// [`Types/SelectionModeFlags`](https://lua-api.factorio.com/latest/types/SelectionModeFlags.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SelectionModeFlags {
    Array(FactorioArray<SelectionModeFlagsUnion>),
    Single(SelectionModeFlagsUnion),
}

/// [`Types/SelectionModeFlags`](https://lua-api.factorio.com/latest/types/SelectionModeFlags.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SelectionModeFlagsUnion {
    Blueprint,
    Deconstruct,
    CancelDeconstruct,
    Items,
    Trees,
    BuildableType,
    Nothing,
    ItemsToPlace,
    AnyEntity,
    AnyTile,
    SameForce,
    NotSameForce,
    Friend,
    Enemy,
    Upgrade,
    CancelUpgrade,
    Downgrade,
    EntityWithHealth,
    IsMilitaryTarget,
    EntityWithOwner,
    AvoidRollingStock,
    AvoidVehicle,
    Controllable,
    ControllableAdd,
    ControllableRemove,
    EntityGhost,
    TileGhost,
}

/// [`Types/CursorBoxType`](https://lua-api.factorio.com/latest/types/CursorBoxType.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CursorBoxType {
    Entity,
    MultiplayerEntity,
    Electricity,
    Copy,
    NotAllowed,
    Pair,
    Logistics,
    TrainVisualization,
    BlueprintSnapRectangle,
    SpidertronRemoteSelected,
    SpidertronRemoteToBeSelected,
}

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
    Multiple(FactorioArray<ItemToPlace>),
}

/// [`Types/CollisionMaskConnector`](https://lua-api.factorio.com/latest/types/CollisionMaskConnector.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct CollisionMaskConnector {
    pub layers: HashMap<CollisionLayerID, bool>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub not_colliding_with_self: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub consider_tile_transitions: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub colliding_with_tiles_only: bool,
}

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
    #[serde(rename = "building-direction-16-way")]
    BuildingDirection16Way,
    FilterDirections,
    GetByUnitNumber,
    BreathsAir,
    NotRepairable,
    NotOnMap,
    NotDeconstructable,
    NotBlueprintable,
    HideAltInfo,
    NotFlammable,
    NoAutomatedItemRemoval,
    NoAutomatedItemInsertion,
    NoCopyPaste,
    NotSelectableInGame,
    NotUpgradable,
    NotInKillStatistics,
    SnapToRailSupportSpot,
    NotInMadeIn,
}

/// [`Types/EntityPrototypeFlags`](https://lua-api.factorio.com/latest/types/EntityPrototypeFlags.html)
pub type EntityPrototypeFlags = FactorioArray<EntityPrototypeFlag>;

/// [`Types/EntityStatus`](https://lua-api.factorio.com/latest/types/EntityStatus.html)
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum EntityStatus {
    Working,
    #[default]
    Normal,
    Ghost,
    NotPluggedInElectricNetwork,
    NetworksConnected,
    NetworksDisconnected,
    NoAmmo,
    WaitingForTargetToBeBuilt,
    WaitingForTrain,
    NoPower,
    LowTemperature,
    Charging,
    Discharging,
    FullyCharged,
    NoFuel,
    NoFood,
    OutOfLogisticNetwork,
    NoRecipe,
    NoIngredients,
    NoInputFluid,
    NoResearchInProgress,
    NoMinableResources,
    LowInputFluid,
    LowPower,
    NotConnectedToRail,
    CantDivideSegments,
    RechargingAfterPowerOutage,
    NoModulesToTransmit,
    DisabledByControlBehavior,
    OpenedByCircuitNetwork,
    ClosedByCircuitNetwork,
    DisabledByScript,
    Disabled,
    TurnedOffDuringDaytime,
    FluidIngredientShortage,
    ItemIngredientShortage,
    FullOutput,
    NotEnoughSpaceInOutput,
    FullBurntResultOutput,
    MarkedForDeconstruction,
    MissingRequiredFluid,
    MissingSciencePacks,
    WaitingForSourceItems,
    WaitingForSpaceInDestination,
    PreparingRocketForLaunch,
    WaitingToLaunchRocket,
    WaitingForSpaceInPlatformHub,
    LaunchingRocket,
    ThrustNotRequired,
    NotEnoughThrust,
    OnTheWay,
    WaitingInOrbit,
    WaitingForRocketToArrive,
    NoPath,
    Broken,
    None,
    Frozen,
    Paused,
    NotConnectedToHubOrPad,
    ComputingNavigation,
    NoFilter,
    WaitingAtStop,
    DestinationStopFull,
    PipelineOverextended,
    NoSpotSeedableByInputs,
    WaitingForPlantsToGrow,
    RecipeNotResearched,
}

/// [`Types/Mirroring`](https://lua-api.factorio.com/latest/types/Mirroring.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Mirroring {
    Horizontal,
    Vertical,
    DiagonalPos,
    DiagonalNeg,
}

/// [`Types/MapPosition`](https://lua-api.factorio.com/latest/types/MapPosition.html)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MapPosition {
    XY { x: f64, y: f64 },
    Tuple(f64, f64),
}

impl MapPosition {
    #[must_use]
    pub const fn x(&self) -> f64 {
        match self {
            Self::Tuple(x, _) | Self::XY { x, .. } => *x,
        }
    }

    #[must_use]
    pub const fn y(&self) -> f64 {
        match self {
            Self::Tuple(_, y) | Self::XY { y, .. } => *y,
        }
    }

    #[must_use]
    pub const fn as_tuple(&self) -> (f64, f64) {
        match self {
            Self::Tuple(x, y) | Self::XY { x, y } => (*x, *y),
        }
    }

    #[must_use]
    pub const fn as_tuple_mut(&mut self) -> (&mut f64, &mut f64) {
        match self {
            Self::Tuple(x, y) | Self::XY { x, y } => (x, y),
        }
    }

    #[must_use]
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

    #[must_use]
    pub fn is_cardinal_neighbor(&self, other: &Self) -> Option<Direction> {
        const CARDINAL_MAX: f64 = 1.125;
        const CARDINAL_MIN: f64 = 0.875;
        const SHEAR_MAX: f64 = 0.125;

        self.is_cardinal_neighbor_internal(other, CARDINAL_MAX, CARDINAL_MIN, SHEAR_MAX)
    }

    #[must_use]
    pub fn is_2long_cardinal_neighbor(&self, other: &Self) -> Option<Direction> {
        const CARDINAL_MAX: f64 = 2.125;
        const CARDINAL_MIN: f64 = 1.875;
        const SHEAR_MAX: f64 = 0.125;

        self.is_cardinal_neighbor_internal(other, CARDINAL_MAX, CARDINAL_MIN, SHEAR_MAX)
    }

    #[must_use]
    pub fn is_2wide_cardinal_neighbor(&self, other: &Self) -> Option<Direction> {
        const CARDINAL_MAX: f64 = 1.125;
        const CARDINAL_MIN: f64 = 0.875;
        const SHEAR_MAX: f64 = 0.875;

        self.is_cardinal_neighbor_internal(other, CARDINAL_MAX, CARDINAL_MIN, SHEAR_MAX)
    }

    #[must_use]
    pub fn distance_to(&self, other: &Self) -> f64 {
        let (dx, dy) = (self - other).as_tuple();
        dx.hypot(dy)
    }

    #[must_use]
    pub const fn center_to(&self, other: &Self) -> Self {
        let (x1, y1) = self.as_tuple();
        let (x2, y2) = other.as_tuple();

        Self::Tuple(x1.midpoint(x2), y1.midpoint(y2))
    }

    #[must_use]
    pub fn rad_orientation_to(&self, other: &Self) -> f64 {
        let (x1, y1) = self.as_tuple();
        let (x2, y2) = other.as_tuple();

        (y2 - y1).atan2(x2 - x1)
    }

    #[must_use]
    pub fn orientation_to(&self, other: &Self) -> RealOrientation {
        let res = self.rad_orientation_to(other) / std::f64::consts::TAU;
        RealOrientation::new((res + 1.0) % 1.0)
    }
}

impl std::fmt::Display for MapPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x, y) = self.as_tuple();

        write!(f, "({x}, {y})")
    }
}

impl Default for MapPosition {
    fn default() -> Self {
        Self::Tuple(Default::default(), Default::default())
    }
}

impl PartialOrd for MapPosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let (x1, y1) = self.as_tuple();
        let (x2, y2) = other.as_tuple();

        y1.partial_cmp(&y2).map_or_else(
            || x1.partial_cmp(&x2),
            |res| match res {
                std::cmp::Ordering::Equal => x1.partial_cmp(&x2),
                std::cmp::Ordering::Less | std::cmp::Ordering::Greater => Some(res),
            },
        )
    }
}

impl PartialEq for MapPosition {
    fn eq(&self, other: &Self) -> bool {
        let (x1, y1) = self.as_tuple();
        let (x2, y2) = other.as_tuple();

        x1 == x2 && y1 == y2
    }
}

impl Eq for MapPosition {}

impl Hash for MapPosition {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let (x, y) = self.as_tuple();
        x.to_bits().hash(state);
        y.to_bits().hash(state);
    }
}

impl From<Vector> for MapPosition {
    fn from(vector: Vector) -> Self {
        let (x, y) = vector.as_tuple();

        Self::Tuple(x, y)
    }
}

impl std::ops::Add for MapPosition {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (x1, y1) = self.as_tuple();
        let (x2, y2) = rhs.as_tuple();

        Self::Tuple(x1 + x2, y1 + y2)
    }
}

impl std::ops::Add for &MapPosition {
    type Output = MapPosition;

    fn add(self, rhs: Self) -> Self::Output {
        let (x1, y1) = self.as_tuple();
        let (x2, y2) = rhs.as_tuple();

        MapPosition::Tuple(x1 + x2, y1 + y2)
    }
}

impl std::ops::Add<&Self> for MapPosition {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        let (x1, y1) = self.as_tuple();
        let (x2, y2) = rhs.as_tuple();

        Self::Tuple(x1 + x2, y1 + y2)
    }
}

impl std::ops::Sub for MapPosition {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let (x1, y1) = self.as_tuple();
        let (x2, y2) = rhs.as_tuple();

        Self::Tuple(x1 - x2, y1 - y2)
    }
}

impl std::ops::Sub for &MapPosition {
    type Output = MapPosition;

    fn sub(self, rhs: Self) -> Self::Output {
        let (x1, y1) = self.as_tuple();
        let (x2, y2) = rhs.as_tuple();

        MapPosition::Tuple(x1 - x2, y1 - y2)
    }
}

impl std::ops::Sub<&Self> for MapPosition {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        let (x1, y1) = self.as_tuple();
        let (x2, y2) = rhs.as_tuple();

        Self::Tuple(x1 - x2, y1 - y2)
    }
}

impl std::ops::Mul<f64> for MapPosition {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let (x, y) = self.as_tuple();

        Self::Tuple(x * rhs, y * rhs)
    }
}

impl std::ops::Div<f64> for MapPosition {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let (x, y) = self.as_tuple();

        Self::Tuple(x / rhs, y / rhs)
    }
}

impl std::ops::AddAssign for MapPosition {
    fn add_assign(&mut self, rhs: Self) {
        let (x1, y1) = self.as_tuple_mut();
        let (x2, y2) = rhs.as_tuple();

        *x1 += x2;
        *y1 += y2;
    }
}

impl std::ops::SubAssign for MapPosition {
    fn sub_assign(&mut self, rhs: Self) {
        let (x1, y1) = self.as_tuple_mut();
        let (x2, y2) = rhs.as_tuple();

        *x1 -= x2;
        *y1 -= y2;
    }
}

impl std::ops::MulAssign<f64> for MapPosition {
    fn mul_assign(&mut self, rhs: f64) {
        let (x, y) = self.as_tuple_mut();

        *x *= rhs;
        *y *= rhs;
    }
}

impl std::ops::DivAssign<f64> for MapPosition {
    fn div_assign(&mut self, rhs: f64) {
        let (x, y) = self.as_tuple_mut();

        *x /= rhs;
        *y /= rhs;
    }
}

/// [`Types/BoundingBox`](https://lua-api.factorio.com/latest/types/BoundingBox.html)
#[derive(Debug, Clone, Default, Serialize)]
pub struct BoundingBox(pub MapPosition, pub MapPosition);

impl BoundingBox {
    #[must_use]
    pub const fn top_left(&self) -> &MapPosition {
        &self.0
    }

    #[must_use]
    pub const fn bottom_right(&self) -> &MapPosition {
        &self.1
    }

    #[must_use]
    pub const fn top(&self) -> f64 {
        self.0.y()
    }

    #[must_use]
    pub const fn bottom(&self) -> f64 {
        self.1.y()
    }

    #[must_use]
    pub const fn left(&self) -> f64 {
        self.0.x()
    }

    #[must_use]
    pub const fn right(&self) -> f64 {
        self.1.x()
    }

    #[must_use]
    pub fn width(&self) -> f64 {
        self.right() - self.left()
    }

    #[must_use]
    pub fn height(&self) -> f64 {
        self.bottom() - self.top()
    }

    #[must_use]
    pub const fn center(&self) -> MapPosition {
        let (x1, y1) = self.0.as_tuple();
        let (x2, y2) = self.1.as_tuple();

        MapPosition::Tuple(x1.midpoint(x2), y1.midpoint(y2))
    }
}

impl<'de> Deserialize<'de> for BoundingBox {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BoundingBoxVisitor;

        impl<'de> serde::de::Visitor<'de> for BoundingBoxVisitor {
            type Value = BoundingBox;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a BoundingBox")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                use serde::de::Error;

                #[derive(Deserialize)]
                #[serde(field_identifier, rename_all = "snake_case")]
                enum Field {
                    LeftTop,
                    RightBottom,
                    Orientation, // unused
                }

                let mut left_top = None;
                let mut right_bottom = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::LeftTop => {
                            if left_top.is_some() {
                                return Err(serde::de::Error::duplicate_field("left_top"));
                            }
                            left_top = Some(map.next_value()?);
                        }
                        Field::RightBottom => {
                            if right_bottom.is_some() {
                                return Err(serde::de::Error::duplicate_field("right_bottom"));
                            }
                            right_bottom = Some(map.next_value()?);
                        }
                        Field::Orientation => {}
                    }
                }

                let left_top = left_top.ok_or_else(|| Error::missing_field("left_top"))?;
                let right_bottom =
                    right_bottom.ok_or_else(|| Error::missing_field("right_bottom"))?;

                Ok(BoundingBox(left_top, right_bottom))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                use serde::de::Error;

                let left_top = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(0, &self))?;
                let right_bottom = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(1, &self))?;

                Ok(BoundingBox(left_top, right_bottom))
            }
        }

        deserializer.deserialize_any(BoundingBoxVisitor)
    }
}

/// [`Types/Direction`](https://lua-api.factorio.com/latest/types/Direction.html)
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize_repr,
    Deserialize_repr,
)]
#[repr(u8)]
pub enum Direction {
    #[default]
    North = 0,
    NorthNorthEast = 1,
    NorthEast = 2,
    EastNorthEast = 3,
    East = 4,
    EastSouthEast = 5,
    SouthEast = 6,
    SouthSouthEast = 7,
    South = 8,
    SouthSouthWest = 9,
    SouthWest = 10,
    WestSouthWest = 11,
    West = 12,
    WestNorthWest = 13,
    NorthWest = 14,
    NorthNorthWest = 15,
}

impl Direction {
    pub const COUNT: usize = 16;
    pub const ALL: [Self; Self::COUNT] = [
        Self::North,
        Self::NorthNorthEast,
        Self::NorthEast,
        Self::EastNorthEast,
        Self::East,
        Self::EastSouthEast,
        Self::SouthEast,
        Self::SouthSouthEast,
        Self::South,
        Self::SouthSouthWest,
        Self::SouthWest,
        Self::WestSouthWest,
        Self::West,
        Self::WestNorthWest,
        Self::NorthWest,
        Self::NorthNorthWest,
    ];

    #[must_use]
    pub const fn flip(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::NorthNorthEast => Self::SouthSouthWest,
            Self::NorthEast => Self::SouthWest,
            Self::EastNorthEast => Self::WestSouthWest,
            Self::East => Self::West,
            Self::EastSouthEast => Self::WestNorthWest,
            Self::SouthEast => Self::NorthWest,
            Self::SouthSouthEast => Self::NorthNorthWest,
            Self::South => Self::North,
            Self::SouthSouthWest => Self::NorthNorthEast,
            Self::SouthWest => Self::NorthEast,
            Self::WestSouthWest => Self::EastNorthEast,
            Self::West => Self::East,
            Self::WestNorthWest => Self::EastSouthEast,
            Self::NorthWest => Self::SouthEast,
            Self::NorthNorthWest => Self::SouthSouthEast,
        }
    }

    /// Rotate the provided vector to fit the direction.
    /// The vector is assumed to be in the north direction.
    #[must_use]
    pub fn rotate_vector(self, vector: Vector) -> Vector {
        let (x_fac, y_fac, swap) = match self {
            Self::North => (1.0, 1.0, false),
            Self::East => (-1.0, 1.0, true),
            Self::South => (-1.0, -1.0, false),
            Self::West => (1.0, -1.0, true),
            _ => todo!("rotation for non-cardinal directions not yet implemented"),
        };

        let (x, y) = if swap {
            (vector.y(), vector.x())
        } else {
            (vector.x(), vector.y())
        };

        Vector::new(x * x_fac, y * y_fac)
    }

    #[must_use]
    pub fn mirror_vector(self, vector: Vector) -> Vector {
        match self {
            Self::North | Self::South => vector.flip_x(),
            Self::East | Self::West => vector.flip_y(),
            _ => vector, // diagonal mirrors are not supported but this is a safe fallback
        }
    }

    #[must_use]
    pub const fn is_straight(&self, other: &Self) -> bool {
        matches!(self, other) || matches!(self.flip(), other)
    }

    #[must_use]
    pub const fn is_right_angle(&self, other: &Self) -> bool {
        match self {
            Self::North | Self::South => matches!(other, Self::East | Self::West),
            Self::East | Self::West => matches!(other, Self::North | Self::South),
            Self::NorthEast | Self::SouthWest => matches!(other, Self::NorthWest | Self::SouthEast),
            Self::NorthWest | Self::SouthEast => matches!(other, Self::NorthEast | Self::SouthWest),
            Self::NorthNorthEast | Self::SouthSouthWest => {
                matches!(other, Self::EastSouthEast | Self::WestNorthWest)
            }
            Self::NorthNorthWest | Self::SouthSouthEast => {
                matches!(other, Self::EastNorthEast | Self::WestSouthWest)
            }
            Self::EastNorthEast | Self::WestSouthWest => {
                matches!(other, Self::NorthNorthWest | Self::SouthSouthEast)
            }
            Self::EastSouthEast | Self::WestNorthWest => {
                matches!(other, Self::NorthNorthEast | Self::SouthSouthWest)
            }
        }
    }

    #[must_use]
    pub const fn to_orientation(&self) -> RealOrientation {
        let val = match self {
            Self::North => 0.0,
            Self::NorthNorthEast => 0.0625,
            Self::NorthEast => 0.125,
            Self::EastNorthEast => 0.1875,
            Self::East => 0.25,
            Self::EastSouthEast => 0.3125,
            Self::SouthEast => 0.375,
            Self::SouthSouthEast => 0.4375,
            Self::South => 0.5,
            Self::SouthSouthWest => 0.5625,
            Self::SouthWest => 0.625,
            Self::WestSouthWest => 0.6875,
            Self::West => 0.75,
            Self::WestNorthWest => 0.8125,
            Self::NorthWest => 0.875,
            Self::NorthNorthWest => 0.9375,
        };

        RealOrientation::new(val)
    }

    #[must_use]
    pub fn is_default(other: &Self) -> bool {
        other == &Self::default()
    }

    #[must_use]
    pub const fn right90(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::NorthNorthEast => Self::EastSouthEast,
            Self::NorthEast => Self::SouthEast,
            Self::EastNorthEast => Self::SouthSouthEast,
            Self::East => Self::South,
            Self::EastSouthEast => Self::SouthSouthWest,
            Self::SouthEast => Self::SouthWest,
            Self::SouthSouthEast => Self::WestSouthWest,
            Self::South => Self::West,
            Self::SouthSouthWest => Self::WestNorthWest,
            Self::SouthWest => Self::NorthWest,
            Self::WestSouthWest => Self::NorthNorthWest,
            Self::West => Self::North,
            Self::WestNorthWest => Self::NorthNorthEast,
            Self::NorthWest => Self::NorthEast,
            Self::NorthNorthWest => Self::EastNorthEast,
        }
    }

    #[must_use]
    pub const fn get_offset(&self) -> Vector {
        match self {
            Self::North => Vector::new(0.0, -1.0),
            Self::NorthEast => Vector::new(1.0, -1.0),
            Self::East => Vector::new(1.0, 0.0),
            Self::SouthEast => Vector::new(1.0, 1.0),
            Self::South => Vector::new(0.0, 1.0),
            Self::SouthWest => Vector::new(-1.0, 1.0),
            Self::West => Vector::new(-1.0, 0.0),
            Self::NorthWest => Vector::new(-1.0, -1.0),
            _ => todo!(),
        }
    }

    #[must_use]
    pub const fn as_4way_idx(&self) -> Option<usize> {
        match self {
            Self::North => Some(0),
            Self::East => Some(1),
            Self::South => Some(2),
            Self::West => Some(3),
            _ => None,
        }
    }
}

impl TryFrom<u8> for Direction {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::North),
            1 => Ok(Self::NorthNorthEast),
            2 => Ok(Self::NorthEast),
            3 => Ok(Self::EastNorthEast),
            4 => Ok(Self::East),
            5 => Ok(Self::EastSouthEast),
            6 => Ok(Self::SouthEast),
            7 => Ok(Self::SouthSouthEast),
            8 => Ok(Self::South),
            9 => Ok(Self::SouthSouthWest),
            10 => Ok(Self::SouthWest),
            11 => Ok(Self::WestSouthWest),
            12 => Ok(Self::West),
            13 => Ok(Self::WestNorthWest),
            14 => Ok(Self::NorthWest),
            15 => Ok(Self::NorthNorthWest),
            _ => Err(()),
        }
    }
}

/// Single element of [`Types/Resistances`](https://lua-api.factorio.com/latest/types/Resistances.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct Resistance {
    #[serde(rename = "type")]
    pub type_: DamageTypeID,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub decrease: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub percent: f64,
}

/// [`Types/Resistances`](https://lua-api.factorio.com/latest/types/Resistances.html)
pub type Resistances = FactorioArray<Resistance>;

/// [`Types/RadiusVisualisationSpecification`](https://lua-api.factorio.com/latest/types/RadiusVisualisationSpecification.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RadiusVisualisationSpecification {
    pub sprite: Option<Sprite>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub distance: f64,

    pub offset: Option<Vector>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_in_cursor: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_on_selection: bool,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
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
    #[serde(default, rename = "type", skip_serializing_if = "helper::is_default")]
    pub type_: LightDefinitionType,

    pub picture: Option<Sprite>, // mandatory for oriented

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub rotation_shift: RealOrientation,

    pub intensity: f32,
    pub size: f32,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub source_orientation_offset: RealOrientation,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub add_perspective: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub flicker_interval: u8,

    #[serde(default = "helper::f32_1", skip_serializing_if = "helper::is_1_f32")]
    pub flicker_min_modifier: f32,

    pub flicker_max_modifier: Option<f32>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub offset_flicker: bool,

    pub shift: Option<Vector>,

    #[serde(default = "Color::white", skip_serializing_if = "Color::is_white")]
    pub color: Color,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub minimum_darkness: f64,
}

/// [`Types/LightDefinition`](https://lua-api.factorio.com/latest/types/LightDefinition.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LightDefinition {
    Struct(Box<LightDefinitionData>),
    Array(FactorioArray<LightDefinitionData>),
}

/// [`Types/BoxSpecification`](https://lua-api.factorio.com/latest/types/BoxSpecification.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct BoxSpecification {
    pub sprite: Sprite,

    #[serde(default, skip_serializing_if = "serde_helper::is_default")]
    pub is_whole_box: bool,

    // TODO: model mandatory depending on `is_whole_box`
    pub side_length: Option<f64>,
    pub side_height: Option<f64>,
    pub max_side_length: Option<f64>,
}

/// [`Types/EntityBuildAnimationPiece`](https://lua-api.factorio.com/latest/types/EntityBuildAnimationPiece.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityBuildAnimationPiece {
    pub top: Animation,
    pub body: Animation,
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
    pub north: FactorioArray<PumpConnectorGraphicsAnimation>,
    pub east: FactorioArray<PumpConnectorGraphicsAnimation>,
    pub south: FactorioArray<PumpConnectorGraphicsAnimation>,
    pub west: FactorioArray<PumpConnectorGraphicsAnimation>,
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
    pub armors: FactorioArray<String>,
}

/// [`Types/CraftingMachineGraphicsSet`](https://lua-api.factorio.com/latest/types/CraftingMachineGraphicsSet.html)
pub type CraftingMachineGraphicsSet = WorkingVisualisations<CraftingMachineGraphicsSetData>;

/// [`Types/CraftingMachineGraphicsSet`](https://lua-api.factorio.com/latest/types/CraftingMachineGraphicsSet.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct CraftingMachineGraphicsSetData {
    pub frozen_patch: Option<Sprite4Way>,

    #[serde(default = "helper::f32_05", skip_serializing_if = "helper::is_05_f32")]
    pub animation_progress: f32,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub reset_animation_when_frozen: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum GuiMode {
    All,
    None,
    Admins,
}

impl GuiMode {
    #[must_use]
    pub const fn all() -> Self {
        Self::All
    }

    #[must_use]
    pub const fn none() -> Self {
        Self::None
    }

    #[must_use]
    pub const fn admins() -> Self {
        Self::Admins
    }

    #[must_use]
    pub const fn is_all(value: &Self) -> bool {
        matches!(value, Self::All)
    }

    #[must_use]
    pub const fn is_none(value: &Self) -> bool {
        matches!(value, Self::None)
    }

    #[must_use]
    pub const fn is_admins(value: &Self) -> bool {
        matches!(value, Self::Admins)
    }
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
    pub connections: FactorioArray<HeatConnection>,
}

impl HeatBuffer {
    #[must_use]
    pub fn connection_points(&self) -> Vec<MapPosition> {
        self.connections
            .iter()
            .map(|c| {
                let offset: MapPosition = (c.direction.get_offset()).into();
                c.position + offset
            })
            .collect()
    }
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
    #[allow(clippy::fn_params_excessive_bools)]
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
    #[must_use]
    pub const fn get(&self, connections: ConnectedDirections) -> &SpriteVariations {
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
    #[must_use]
    pub const fn all() -> Self {
        Self::All
    }

    #[must_use]
    pub const fn ally() -> Self {
        Self::Ally
    }

    #[must_use]
    pub const fn same() -> Self {
        Self::Same
    }

    #[must_use]
    pub const fn enemy() -> Self {
        Self::Enemy
    }

    #[must_use]
    pub const fn friend() -> Self {
        Self::Friend
    }

    #[must_use]
    pub const fn not_same() -> Self {
        Self::NotSame
    }

    #[must_use]
    pub const fn not_friend() -> Self {
        Self::NotFriend
    }

    #[must_use]
    pub const fn is_all(value: &Self) -> bool {
        matches!(value, Self::All)
    }

    #[must_use]
    pub const fn is_ally(value: &Self) -> bool {
        matches!(value, Self::Ally)
    }

    #[must_use]
    pub const fn is_same(value: &Self) -> bool {
        matches!(value, Self::Same)
    }

    #[must_use]
    pub const fn is_enemy(value: &Self) -> bool {
        matches!(value, Self::Enemy)
    }

    #[must_use]
    pub const fn is_friend(value: &Self) -> bool {
        matches!(value, Self::Friend)
    }

    #[must_use]
    pub const fn is_not_same(value: &Self) -> bool {
        matches!(value, Self::NotSame)
    }

    #[must_use]
    pub const fn is_not_friend(value: &Self) -> bool {
        matches!(value, Self::NotFriend)
    }
}

/// [`Types/MiningDrillGraphicsSet`](https://lua-api.factorio.com/latest/types/MiningDrillGraphicsSet.html)
pub type MiningDrillGraphicsSet = WorkingVisualisations<MiningDrillGraphicsSetData>;

/// [`Types/MiningDrillGraphicsSet`](https://lua-api.factorio.com/latest/types/MiningDrillGraphicsSet.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct MiningDrillGraphicsSetData {
    pub frozen_patch: Option<Sprite4Way>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub reset_animation_when_frozen: bool,

    pub circuit_connector_layer: Option<CircuitConnectorLayer>, // TODO: fix that only the internal members need to be optional
    pub circuit_connector_secondary_draw_order: Option<CircuitConnectorSecondaryDrawOrder>, // TODO: fix that only the internal members need to be optional

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub drilling_vertical_movement_duration: u16,

    #[serde(default = "helper::f32_1", skip_serializing_if = "helper::is_1_f32")]
    pub animation_progress: f32,
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

// Comparator variants
#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum Comparator {
    #[default]
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

    #[serde(rename = "?", other)]
    Unknown,
}

// https://lua-api.factorio.com/latest/concepts/ArithmeticCombinatorParameterOperation.html
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
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

    #[serde(rename = "?", other)]
    Unknown,
}

// https://lua-api.factorio.com/latest/concepts/SelectorCombinatorParameterOperation.html
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SelectorOperation {
    Select {
        select_max: bool,
    },
    Count,
    Random,
    QualityTransfer,
    RocketCapacity,
    StackSize,
    QualityFilter,

    #[serde(rename = "?", other)]
    Unknown,
}

/// [`Types/FluidAmount`](https://lua-api.factorio.com/latest/types/FluidAmount.html)
pub type FluidAmount = f64;

/// [`Types/Weight`](https://lua-api.factorio.com/latest/types/Weight.html)
pub type Weight = f64;

/// [`Types/LogisticFilterIndex`](https://lua-api.factorio.com/latest/types/LogisticFilterIndex.html)
pub type LogisticFilterIndex = u16;

/// [`Types/PerceivedPerformance`](https://lua-api.factorio.com/latest/types/PerceivedPerformance.html)
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PerceivedPerformance {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub minimum: f64,

    #[serde(
        default = "helper::f64_max",
        skip_serializing_if = "helper::is_max_f64"
    )]
    pub maximum: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub performance_to_activity_rate: f64,
}

impl Default for PerceivedPerformance {
    fn default() -> Self {
        Self {
            minimum: 0.0,
            maximum: f64::MAX,
            performance_to_activity_rate: 1.0,
        }
    }
}

/// [`Types/ProductionHealthEffect`](https://lua-api.factorio.com/latest/types/ProductionHealthEffect.html)
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ProductionHealthEffect {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub producing: f32,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub not_producing: f32,
}

/// [`Types/CargoStationParameters`](https://lua-api.factorio.com/latest/types/CargoStationParameters.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct CargoStationParameters {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub prefer_packed_cargo_units: bool,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hatch_definitions: FactorioArray<CargoHatchDefinition>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub giga_hatch_definitions: FactorioArray<GigaCargoHatchDefinition>,
}

/// [`Types/CargoHatchDefinition`](https://lua-api.factorio.com/latest/types/CargoHatchDefinition.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct CargoHatchDefinition {
    pub hatch_graphics: Option<Animation>,
    pub hatch_render_layer: Option<RenderLayer>,
    pub entering_render_layer: Option<RenderLayer>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub offset: Vector,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub pod_shadow_offset: Vector,

    #[serde(default = "helper::f32_n1", skip_serializing_if = "helper::is_n1_f32")]
    pub sky_slice_height: f32,
    #[serde(default = "helper::f32_1", skip_serializing_if = "helper::is_1_f32")]
    pub slice_height: f32,
    #[serde(default = "helper::f32_1", skip_serializing_if = "helper::is_1_f32")]
    pub travel_height: f32,
    #[serde(
        default = "helper::u32_120",
        skip_serializing_if = "helper::is_120_u32",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub busy_timeout_ticks: u32,
    #[serde(
        default = "helper::u32_80",
        skip_serializing_if = "helper::is_80_u32",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub hatch_opening_ticks: u32,

    // pub opening_sound: Option<InterruptibleSound>,
    // pub closing_sound: Option<InterruptibleSound>,
    pub cargo_unit_entity_to_spawn: Option<EntityID>,
    pub illumination_graphic_index: Option<u32>,
}

/// [`Types/GigaCargoHatchDefinition`](https://lua-api.factorio.com/latest/types/GigaCargoHatchDefinition.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct GigaCargoHatchDefinition {
    pub hatch_graphics_back: Option<Animation>,
    pub hatch_graphics_front: Option<Animation>,
    pub hatch_render_layer_back: Option<RenderLayer>,
    pub hatch_render_layer_front: Option<RenderLayer>,
    pub covered_hatches: FactorioArray<u32>,
    // pub opening_sound: Option<InterruptibleSound>,
    // pub closing_sound: Option<InterruptibleSound>,
}

/// [`Types/WaterReflectionDefinition`](https://lua-api.factorio.com/latest/types/WaterReflectionDefinition.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct WaterReflectionDefinition {
    pub pictures: Option<SpriteVariations>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub orientation_to_variation: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub rotate: bool,
}

/// [`Types/ChargableGraphics`](https://lua-api.factorio.com/latest/types/ChargableGraphics.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ChargableGraphics {
    pub picture: Option<Sprite>,
    pub charge_animation: Option<Animation>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub charge_animation_is_looped: bool,

    pub charge_light: Option<LightDefinition>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub charge_cooldown: Option<u16>,

    pub discharge_animation: Option<Animation>,
    pub discharge_light: Option<LightDefinition>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub discharge_cooldown: Option<u16>,
}
