#![allow(
    unused_variables,
    clippy::wildcard_imports,
    clippy::struct_excessive_bools,
    clippy::module_name_repetitions
)]

use std::{collections::HashMap, fmt};

use konst::{primitive::parse_u16, result::unwrap_ctx};

use mod_util::{mod_info::Version, UsedMods};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;

use serde_helper as helper;

#[must_use]
pub const fn targeted_engine_version() -> Version {
    Version::new(
        unwrap_ctx!(parse_u16(env!("CARGO_PKG_VERSION_MAJOR"))),
        unwrap_ctx!(parse_u16(env!("CARGO_PKG_VERSION_MINOR"))),
        unwrap_ctx!(parse_u16(env!("CARGO_PKG_VERSION_PATCH"))),
    )
}

mod empty_array_fix;
mod energy;
mod graphics;
mod icon;
mod item;
mod module;
mod wire;

pub use empty_array_fix::*;
pub use energy::*;
pub use graphics::*;
pub use icon::*;
pub use item::*;
pub use module::*;
pub use wire::*;

/// [`Types/AmmoCategoryID`](https://lua-api.factorio.com/latest/types/AmmoCategoryID.html)
pub type AmmoCategoryID = String;

/// [`Types/AmmoType`](https://lua-api.factorio.com/latest/types/AmmoType.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct AmmoType {
    pub category: AmmoCategoryID,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub clamp_position: bool,

    pub energy_consumption: Option<Energy>,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub range_modifier: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub cooldown_modifier: f64,

    #[serde(default = "helper::f32_1", skip_serializing_if = "helper::is_1_f32")]
    pub consumption_modifier: f32,

    pub target_type: Option<AmmoTypeTargetType>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub source_type: AmmoSourceType,
    // not implemented
    // pub action: Option<Trigger>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
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

    // default is value of cooldown property
    pub movement_slow_down_cool_down: Option<f32>,

    #[serde(default = "helper::f32_1", skip_serializing_if = "helper::is_1_f32")]
    pub movement_slow_down_factor: f32,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub activation_type: BaseAttackParametersActivationType,

    pub animation: Option<RotatedAnimation>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub use_shooter_direction: bool,
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
        #[serde(default, skip_serializing_if = "Vector::is_0_vector")]
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
        fluid_consumption: f32,

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
    pub _type: FluidID,

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
        r: Option<f64>,
        g: Option<f64>,
        b: Option<f64>,
        a: Option<f64>,
    },
    RGB(f64, f64, f64),
    RGBA(f64, f64, f64, f64),
}

impl Color {
    #[must_use]
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
    pub fn is_0_vector(value: &Self) -> bool {
        value.x() == 0.0 && value.y() == 0.0
    }
}

impl Default for Vector {
    fn default() -> Self {
        Self::Tuple(Default::default(), Default::default())
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
            println!("Mod {mod_name} not found");
            return None;
        };

        let file_data = match m.get_file(sprite_path) {
            Ok(d) => d,
            Err(e) => {
                println!("Error loading {filename}: {e}");
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
pub struct RealOrientation(f64);

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
        fuel_categories: FactorioArray<FuelCategoryID>,
    },
}

/// [`Types/TileID`](https://lua-api.factorio.com/latest/types/TileID.html)
pub type TileID = String;

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
        positions: FactorioArray<Vector>,

        #[serde(
            default,
            skip_serializing_if = "helper::is_default",
            deserialize_with = "helper::truncating_deserializer"
        )]
        max_underground_distance: u32,

        #[serde(default, rename = "type")]
        type_: PipeConnectionType,
    },
    Single {
        position: Vector,

        #[serde(
            default,
            skip_serializing_if = "helper::is_default",
            deserialize_with = "helper::truncating_deserializer"
        )]
        max_underground_distance: u32,

        #[serde(default, rename = "type")]
        type_: PipeConnectionType,
    },
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
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
    pub pipe_connections: FactorioArray<PipeConnectionDefinition>,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub base_area: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub base_level: f32,

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

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub production_type: FluidBoxProductionType,

    #[serde(flatten)]
    pub secondary_draw_order: Option<FluidBoxSecondaryDrawOrders>,
}

/// [`Types/RecipeID`](https://lua-api.factorio.com/latest/types/RecipeID.html)
pub type RecipeID = String;

/// [`Types/RecipeCategoryID`](https://lua-api.factorio.com/latest/types/RecipeCategoryID.html)
pub type RecipeCategoryID = String;

/// [`Types/EquipmentGridID`](https://lua-api.factorio.com/latest/types/EquipmentGridID.html)
pub type EquipmentGridID = String;

/// [`Types/EquipmentID`](https://lua-api.factorio.com/latest/types/EquipmentID.html)
pub type EquipmentID = String;

/// [`Types/ResourceCategoryID`](https://lua-api.factorio.com/latest/types/ResourceCategoryID.html)
pub type ResourceCategoryID = String;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SignalIDConnector {
    Virtual { name: VirtualSignalID },
    Item { name: ItemID },
    Fluid { name: FluidID },
}

/// [`Types/SelectionModeFlags`](https://lua-api.factorio.com/latest/types/SelectionModeFlags.html)
pub type SelectionModeFlags = FactorioArray<SelectionModeFlagsUnion>;

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
    EntityWithForce,
    IsMilitaryTarget,
    EntityWithOwner,
    AvoidRollingStock,
    EntityGhost,
    TileGhost,
}

/// [`Types/CursorBoxType`](https://lua-api.factorio.com/latest/types/CursorBoxType.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CursorBoxType {
    Entity,
    Electricity,
    Copy,
    NotAllowed,
    Pair,
    Logistics,
    TrainVisualizations,
    BlueprintSnapRectangle,
}

/// [`Types/MouseCursorID`](https://lua-api.factorio.com/latest/types/MouseCursorID.html)
pub type MouseCursorID = String;

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

/// [`Types/CollisionMask`](https://lua-api.factorio.com/latest/types/CollisionMask.html)
pub type CollisionMask = FactorioArray<String>;

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
    FastReplaceableNoCrossTypeWhileMoving,
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

/// [`Types/EntityPrototypeFlags`](https://lua-api.factorio.com/latest/types/EntityPrototypeFlags.html)
pub type EntityPrototypeFlags = FactorioArray<EntityPrototypeFlag>;

/// [`Types/MapPosition`](https://lua-api.factorio.com/latest/types/MapPosition.html)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub fn as_tuple_mut(&mut self) -> (&mut f64, &mut f64) {
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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
}

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
    #[must_use]
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
    #[must_use]
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
            (vector.y(), vector.x())
        } else {
            (vector.x(), vector.y())
        };

        Vector::new(x * x_fac, y * y_fac)
    }

    #[must_use]
    pub const fn is_straight(&self, other: &Self) -> bool {
        match self {
            Self::NorthEast | Self::SouthWest => matches!(other, Self::NorthEast | Self::SouthWest),
            Self::NorthWest | Self::SouthEast => matches!(other, Self::NorthWest | Self::SouthEast),
            Self::North | Self::South => matches!(other, Self::North | Self::South),
            Self::East | Self::West => matches!(other, Self::East | Self::West),
        }
    }

    #[must_use]
    pub const fn is_right_angle(&self, other: &Self) -> bool {
        match self {
            Self::NorthEast | Self::SouthWest => matches!(other, Self::NorthWest | Self::SouthEast),
            Self::NorthWest | Self::SouthEast => matches!(other, Self::NorthEast | Self::SouthWest),
            Self::North | Self::South => matches!(other, Self::East | Self::West),
            Self::East | Self::West => matches!(other, Self::North | Self::South),
        }
    }

    #[must_use]
    pub const fn to_orientation(&self) -> RealOrientation {
        let val = match self {
            Self::North => 0.0,
            Self::NorthEast => 0.125,
            Self::East => 0.25,
            Self::SouthEast => 0.375,
            Self::South => 0.5,
            Self::SouthWest => 0.625,
            Self::West => 0.75,
            Self::NorthWest => 0.875,
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
            Self::NorthEast => Self::SouthEast,
            Self::East => Self::South,
            Self::SouthEast => Self::SouthWest,
            Self::South => Self::West,
            Self::SouthWest => Self::NorthWest,
            Self::West => Self::North,
            Self::NorthWest => Self::NorthEast,
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
        }
    }
}

impl TryFrom<u8> for Direction {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::North),
            1 => Ok(Self::NorthEast),
            2 => Ok(Self::East),
            3 => Ok(Self::SouthEast),
            4 => Ok(Self::South),
            5 => Ok(Self::SouthWest),
            6 => Ok(Self::West),
            7 => Ok(Self::NorthWest),
            _ => Err(()),
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

    pub intensity: f64,
    pub size: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub source_orientation_offset: RealOrientation,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub add_perspective: bool,

    pub shift: Option<Vector>,
    pub color: Option<Color>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub minimum_darkness: f64,
}

/// [`Types/LightDefinition`](https://lua-api.factorio.com/latest/types/LightDefinition.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LightDefinition {
    Struct(LightDefinitionData),
    Array(FactorioArray<LightDefinitionData>),
}

/// [`Types/BoxSpecification`](https://lua-api.factorio.com/latest/types/BoxSpecification.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct BoxSpecification {
    pub sprite: Sprite,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_whole_box: bool,

    // TODO: model mandatory depending on `is_whole_box`
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
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub secondary_draw_order: i8,

    #[serde(default, skip_serializing_if = "helper::is_default")]
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
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub tier_offset: i32,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub slots: FactorioArray<FactorioArray<BeaconModuleVisualization>>,
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

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub min_animation_progress: f64,

    #[serde(
        default = "helper::f64_1000",
        skip_serializing_if = "helper::is_1000_f64"
    )]
    pub max_animation_progress: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub apply_module_tint: ModuleTint,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub apply_module_tint_to_light: ModuleTint,

    pub no_modules_tint: Option<Color>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub animation_list: FactorioArray<AnimationElement>,

    pub light: Option<LightDefinition>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub module_visualisations: FactorioArray<BeaconModuleVisualizations>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
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
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        // TODO: render module visualisations
        merge_layers(
            &self.animation_list,
            scale,
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
            orientation: RealOrientation::default(),
            override_index: value.index_override,
        }
    }
}

impl RenderableGraphics for TransportBeltAnimationSet {
    type RenderOpts = TransportBeltAnimationSetRenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &UsedMods,
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
            .render(scale, used_mods, image_cache, &index_options.into())
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
        scale: f64,
        used_mods: &UsedMods,
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
            scale,
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
    pub frame_main_scanner_movement_speed: f32,
    pub frame_main_scanner_horizontal_start_shift: Vector,
    pub frame_main_scanner_horizontal_end_shift: Vector,
    pub frame_main_scanner_horizontal_y_scale: f32,
    pub frame_main_scanner_horizontal_rotation: RealOrientation,
    pub frame_main_scanner_vertical_start_shift: Vector,
    pub frame_main_scanner_vertical_end_shift: Vector,
    pub frame_main_scanner_vertical_y_scale: f32,
    pub frame_main_scanner_vertical_rotation: RealOrientation,
    pub frame_main_scanner_cross_horizontal_start_shift: Vector,
    pub frame_main_scanner_cross_horizontal_end_shift: Vector,
    pub frame_main_scanner_cross_horizontal_y_scale: f32,
    pub frame_main_scanner_cross_horizontal_rotation: RealOrientation,
    pub frame_main_scanner_cross_vertical_start_shift: Vector,
    pub frame_main_scanner_cross_vertical_end_shift: Vector,
    pub frame_main_scanner_cross_vertical_y_scale: f32,
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
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        if self.draw_as_light {
            return None;
        }

        self.animation
            .as_ref()?
            .render(scale, used_mods, image_cache, opts)
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
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        match self {
            Self::Single { animation } => {
                animation.render(scale, used_mods, image_cache, &opts.into())
            }
            Self::Cardinal {
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
            .and_then(|a| a.render(scale, used_mods, image_cache, &opts.into())),
        }
    }
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
    pub connections: FactorioArray<HeatConnection>,
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
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct MiningDrillGraphicsSet {
    pub animation: Option<Animation4Way>,
    pub idle_animation: Option<Animation4Way>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub always_draw_idle_animation: bool,

    pub default_recipe_tint: Option<Color>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub working_visualisations: FactorioArray<WorkingVisualisation>,

    pub shift_animation_waypoints: Option<ShiftAnimationWaypoints>,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub shift_animation_waypoint_stop_duration: u16,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub shift_animation_transition_duration: u16,

    pub status_colors: Option<StatusColors>,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
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

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub min_animation_progress: f64, // specified as single precision in docs

    pub circuit_connector_layer: Option<CircuitConnectorLayer>, // TODO: fix that only the internal members need to be optional
    pub circuit_connector_secondary_draw_order: Option<CircuitConnectorSecondaryDrawOrder>, // TODO: fix that only the internal members need to be optional
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
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        // TODO: fix for electric drills
        let mut renders = vec![self
            .idle_animation
            .as_ref()
            .or(self.animation.as_ref())
            .and_then(|a| a.render(scale, used_mods, image_cache, &opts.into()))];

        renders.extend(
            self.working_visualisations
                .iter()
                .map(|wv| wv.render(scale, used_mods, image_cache, &opts.into())),
        );

        merge_renders(&renders, scale)
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

// Comparator variants
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
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
}
