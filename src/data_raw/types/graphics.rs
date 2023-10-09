use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{helper, Color, FileName, Vector};

/// [`Types/SpritePriority`](https://lua-api.factorio.com/latest/types/SpritePriority.html)
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpritePriority {
    ExtraHighNoScale,
    ExtraHigh,
    High,
    #[default]
    Medium,
    Low,
    VeryLow,
    NoAtlas,
}

/// Union used in [`Types/SpriteFlags`](https://lua-api.factorio.com/latest/types/SpriteFlags.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpriteFlag {
    NoCrop,
    NotCompressed,
    AlwaysCompressed,
    Mipmap,
    LinearMinification,
    LinearMagnification,
    LinearMipLevel,
    AlphaMask,
    NoScale,
    Mask,
    Icon,
    Gui,
    GuiIcon,
    Light,
    Terrain,
    TerrainEffectMap,
    Shadow,
    Smoke,
    Decal,
    LowObject,
    TrilinearFiltering,
    #[serde(rename = "group=none")]
    GroupNone,
    #[serde(rename = "group=terrain")]
    GroupTerrain,
    #[serde(rename = "group=shadow")]
    GroupShadow,
    #[serde(rename = "group=smoke")]
    GroupSmoke,
    #[serde(rename = "group=decal")]
    GroupDecal,
    #[serde(rename = "group=low-object")]
    GroupLowObject,
    #[serde(rename = "group=gui")]
    GroupGui,
    #[serde(rename = "group=icon")]
    GroupIcon,
    #[serde(rename = "group=icon-background")]
    GroupIconBackground,
    Compressed,
}

/// [`Types/SpriteFlags`](https://lua-api.factorio.com/latest/types/SpriteFlags.html)
pub type SpriteFlags = Vec<SpriteFlag>;

/// [`Types/SpriteSizeType`](https://lua-api.factorio.com/latest/types/SpriteSizeType.html)
pub type SpriteSizeType = i16;

/// [`Types/BlendMode`](https://lua-api.factorio.com/latest/types/BlendMode.html)
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BlendMode {
    #[default]
    Normal,
    Additive,
    AdditiveSoft,
    Multiplicative,
    MultiplicativeWithAlpha,
    Overwrite,
}

/// [`Types/RenderLayer`](https://lua-api.factorio.com/latest/types/RenderLayer.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RenderLayer {
    WaterTile,
    GroundTile,
    TileTransition,
    Decals,
    LowerRadiusVisualization,
    RadiusVisualization,
    TransportBeltIntegration,
    Resource,
    BuildingSmoke,
    Decorative,
    GroundPatch,
    GroundPatchHigher,
    GroundPatchHigher2,
    Remnants,
    Floor,
    TransportBelt,
    TransportBeltEndings,
    TransportBeltCircuitConnector,
    FloorMechanicsUnderCorpse,
    Corpse,
    FloorMechanics,
    Item,
    LowerObject,
    LowerObjectAboveShadow,
    Object,
    HigherObjectUnder,
    HigherObjectAbove,
    ItemInInserterHand,
    Wires,
    WiresAbove,
    EntityInfoIcon,
    EntityInfoIconAbove,
    Explosion,
    Projectile,
    Smoke,
    AirObject,
    AirEntityInfoIcon,
    LightEffect,
    SelectionBox,
    HigherSelectionBox,
    CollisionSelectionBox,
    Arrow,
    Cursor,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpriteSizeParam {
    Size {
        size: SpriteSizeType,
    },
    Size2 {
        size: (SpriteSizeType, SpriteSizeType),
    },
    Explicit {
        width: SpriteSizeType,
        height: SpriteSizeType,
    },
}

/// [`Types/SpriteParameters`](https://lua-api.factorio.com/latest/types/SpriteParameters.html)
///
/// **MISSING THE `filename` FIELD**
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteParams {
    // TODO: skip serializing if default
    #[serde(default)]
    pub priority: SpritePriority,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub flags: SpriteFlags,

    #[serde(flatten)]
    pub size: SpriteSizeParam,

    // TODO: turn position, x, y into some enum?
    #[serde(default)]
    pub position: Option<(SpriteSizeType, SpriteSizeType)>,
    #[serde(default)]
    pub x: SpriteSizeType,
    #[serde(default)]
    pub y: SpriteSizeType,

    #[serde(default)]
    pub shift: Vector,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub scale: f64,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub draw_as_shadow: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub draw_as_glow: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub draw_as_light: bool,

    #[serde(default, skip_serializing_if = "helper::is_0_u8")]
    pub mipmap_count: u8,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub apply_runtime_tint: bool,

    #[serde(default = "Color::white", skip_serializing_if = "Color::is_white")]
    pub tint: Color,

    // TODO: skip serializing if default
    #[serde(default)]
    pub blend_mode: BlendMode,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub load_in_minimal_mode: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub premul_alpha: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub generate_sdf: bool,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SimpleGraphics<T> {
    Simple {
        filename: FileName,

        #[serde(flatten)]
        data: T,

        #[serde(skip_serializing_if = "Option::is_none")]
        hr_version: Option<Box<Self>>,
    },
    Layered {
        layers: Vec<Self>,
    },
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MultiFileGraphics<Single, Multi> {
    Simple {
        filename: FileName,

        #[serde(flatten)]
        data: Single,

        #[serde(skip_serializing_if = "Option::is_none")]
        hr_version: Option<Box<Self>>,
    },
    MultiFile {
        filenames: Vec<FileName>,

        #[serde(flatten)]
        data: Multi,

        #[serde(skip_serializing_if = "Option::is_none")]
        hr_version: Option<Box<Self>>,
    },
    Layered {
        layers: Vec<Self>,
    },
}

/// [`Types/Sprite`](https://lua-api.factorio.com/latest/types/Sprite.html)
pub type Sprite = SimpleGraphics<SpriteParams>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RotatedSpriteParams {
    #[serde(flatten)]
    pub sprite_params: SpriteParams,

    direction_count: u16,

    #[serde(default, skip_serializing_if = "helper::is_0_u64")]
    lines_per_file: u64,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    axially_symmetrical: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    back_equals_front: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    apply_projection: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    counterclockwise: bool,

    #[serde(default, skip_serializing_if = "helper::is_0_u32")]
    line_length: u32,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    allow_low_quality_rotation: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RotatedSpriteParamsMultiFile {
    #[serde(flatten)]
    pub sprite_params: SpriteParams,

    direction_count: u16,

    lines_per_file: u64,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    axially_symmetrical: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    back_equals_front: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    apply_projection: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    counterclockwise: bool,

    #[serde(default, skip_serializing_if = "helper::is_0_u32")]
    line_length: u32,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    allow_low_quality_rotation: bool,
}

/// [`Types/RotatedSprite`](https://lua-api.factorio.com/latest/types/RotatedSprite.html)
pub type RotatedSprite = MultiFileGraphics<RotatedSpriteParams, RotatedSpriteParamsMultiFile>;

/// [`Types/SpriteNWaySheet`](https://lua-api.factorio.com/latest/types/SpriteNWaySheet.html)
/// variant for `Sprite4Way`
#[derive(Debug, Serialize, Deserialize)]
pub struct Sprite4WaySheet {
    #[serde(flatten)]
    pub sprite_params: SpriteParams,

    #[serde(default = "helper::u32_4", skip_serializing_if = "helper::is_0_u32")]
    pub frames: u32,

    pub hr_version: Option<Box<Self>>,
}

/// [`Types/SpriteNWaySheet`](https://lua-api.factorio.com/latest/types/SpriteNWaySheet.html)
/// variant for `Sprite8Way`
#[derive(Debug, Serialize, Deserialize)]
pub struct Sprite8WaySheet {
    #[serde(flatten)]
    pub sprite_params: SpriteParams,

    #[serde(default = "helper::u32_8", skip_serializing_if = "helper::is_0_u32")]
    pub frames: u32,

    pub hr_version: Option<Box<Self>>,
}

/// [`Types/Sprite4Way`](https://lua-api.factorio.com/latest/types/Sprite4Way.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Sprite4Way {
    Sprite(Sprite),
    Sheets {
        sheets: Vec<Sprite4WaySheet>,
    },
    Sheet {
        sheet: Sprite4WaySheet,
    },
    Directions {
        north: Sprite,
        east: Sprite,
        south: Sprite,
        west: Sprite,
    },
}

/// [`Types/Sprite8Way`](https://lua-api.factorio.com/latest/types/Sprite8Way.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Sprite8Way {
    Sheets {
        sheets: Vec<Sprite8WaySheet>,
    },
    Sheet {
        sheet: Sprite8WaySheet,
    },
    Directions {
        north: Sprite,
        north_east: Sprite,
        east: Sprite,
        south_east: Sprite,
        south: Sprite,
        south_west: Sprite,
        west: Sprite,
        north_west: Sprite,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteSheetParams {
    #[serde(flatten)]
    pub sprite_params: SpriteParams,

    #[serde(default = "helper::u32_1", skip_serializing_if = "helper::is_1_u32")]
    pub variation_count: u32,

    #[serde(default = "helper::u32_1", skip_serializing_if = "helper::is_1_u32")]
    pub repeat_count: u32,

    // TODO: support the default based on variation_count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_length: Option<u32>,
}

/// [`Types/SpriteSheet`](https://lua-api.factorio.com/latest/types/SpriteSheet.html)
pub type SpriteSheet = SimpleGraphics<SpriteSheetParams>;

/// [`Types/SpriteVariations`](https://lua-api.factorio.com/latest/types/SpriteVariations.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpriteVariations {
    Struct { sheet: SpriteSheet },
    SpriteSheet(SpriteSheet),
    Array(Vec<Sprite>),
}

/// [`Types/WaterReflectionDefinition`](https://lua-api.factorio.com/latest/types/WaterReflectionDefinition.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct WaterReflectionDefinition {
    pub pictures: Option<SpriteVariations>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub orientation_to_variation: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub rotate: bool,
}

// ======================= //
// =======[ Tiles ]======= //
// ======================= //

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct TileGraphics<T> {
    pub data: T,
    pub hr_version: Option<Box<Self>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileSpriteParams {
    pub count: u32,
    pub picture: FileName,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub scale: f64,

    #[serde(default, skip_serializing_if = "helper::is_0_i16")]
    pub x: SpriteSizeType,

    #[serde(default, skip_serializing_if = "helper::is_0_i16")]
    pub y: SpriteSizeType,

    #[serde(default, skip_serializing_if = "helper::is_0_u32")]
    pub line_length: u32,
}

/// [`Types/TileSprite`](https://lua-api.factorio.com/latest/types/TileSprite.html)
pub type TileSprite = TileGraphics<TileSpriteParams>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TileSpriteProbabilityParams {
    #[serde(flatten)]
    pub tile_sprite_params: TileSpriteParams,

    pub size: u32,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub probability: f64,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub weights: Vec<f64>,
}

/// [`Types/TileSpriteWithProbability`](https://lua-api.factorio.com/latest/types/TileSpriteWithProbability.html)
pub type TileSpriteWithProbability = TileGraphics<TileSpriteProbabilityParams>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TileTransitionSpriteParams {
    #[serde(flatten)]
    pub tile_sprite_params: TileSpriteParams,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub tall: bool,
}

/// [`Types/TileTransitionSprite`](https://lua-api.factorio.com/latest/types/TileTransitionSprite.html)
pub type TileTransitionSprite = TileGraphics<TileTransitionSpriteParams>;

// ======================== //
// =====[ Animations ]===== //
// ======================== //

/// [`Types/AnimationFrameSequence`](https://lua-api.factorio.com/latest/types/AnimationFrameSequence.html)
pub type AnimationFrameSequence = Vec<u16>;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AnimationRunMode {
    #[default]
    Forward,
    Backward,
    ForwardThenBackward,
}

/// [`Types/Stripe`](https://lua-api.factorio.com/latest/types/Stripe.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct Stripe {
    pub width_in_frames: u32,
    pub height_in_frames: Option<u32>, // TODO: is only optional when used in RotatedAnimation
    pub filename: FileName,

    #[serde(default, skip_serializing_if = "helper::is_0_u32")]
    pub x: u32,

    #[serde(default, skip_serializing_if = "helper::is_0_u32")]
    pub y: u32,
}

/// [`Types/AnimationParameters`](https://lua-api.factorio.com/latest/types/AnimationParameters.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationParameters {
    #[serde(flatten)]
    pub sprite_params: SpriteParams,

    // TODO: skip searializing if default
    #[serde(default)]
    pub run_mode: AnimationRunMode,

    #[serde(default = "helper::u32_1", skip_serializing_if = "helper::is_1_u32")]
    pub frame_count: u32,

    #[serde(default, skip_serializing_if = "helper::is_0_u32")]
    pub line_length: u32,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub animation_speed: f64,

    #[serde(
        default = "helper::f64_max",
        skip_serializing_if = "helper::is_max_f64"
    )]
    pub max_advance: f64,

    #[serde(default = "helper::u8_1", skip_serializing_if = "helper::is_1_u8")]
    pub repeat_count: u8,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub frame_sequence: AnimationFrameSequence,
}

/// [`Types/Animation`](https://lua-api.factorio.com/latest/types/Animation.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Animation {
    Layered {
        layers: Vec<Self>,
    },
    Striped {
        stripes: Vec<Stripe>,

        #[serde(flatten)]
        data: AnimationParameters,

        hr_version: Option<Box<Self>>,
    },
    Simple {
        filename: FileName,

        #[serde(flatten)]
        data: AnimationParameters,

        hr_version: Option<Box<Self>>,
    },
}

/// [`Types/Animation4Way`](https://lua-api.factorio.com/latest/types/Animation4Way.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Animation4Way {
    Animation(Animation),
    Struct {
        north: Animation,
        east: Option<Animation>,
        south: Option<Animation>,
        west: Option<Animation>,
    },
}

/// [`Types/AnimationElement`](https://lua-api.factorio.com/latest/types/AnimationElement.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationElement {
    pub render_layer: Option<RenderLayer>,
    pub secondary_draw_order: Option<i8>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_as_sprite: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub draw_as_light: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub apply_tint: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub always_draw: bool,

    pub animation: Option<Animation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationSheetParameters {
    #[serde(flatten)]
    pub animation_params: AnimationParameters,
    pub variation_count: u32,
}

/// [`Types/AnimationSheet`](https://lua-api.factorio.com/latest/types/AnimationSheet.html)
pub type AnimationSheet = SimpleGraphics<AnimationSheetParameters>;

/// [`Types/AnimationVariations`](https://lua-api.factorio.com/latest/types/AnimationVariations.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnimationVariations {
    Animation(Animation),
    Array(Vec<Animation>),
    Sheet { sheet: AnimationSheet },
    Sheets { sheets: Vec<AnimationSheet> },
}

/// [`Types/ShiftAnimationWaypoints`](https://lua-api.factorio.com/latest/types/ShiftAnimationWaypoints.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct ShiftAnimationWaypoints {
    pub north: Vec<Vector>,
    pub east: Vec<Vector>,
    pub south: Vec<Vector>,
    pub west: Vec<Vector>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RotatedAnimationParameters {
    pub direction_count: u32,

    pub lines_per_file: Option<u32>,
    pub slice: Option<u32>,

    #[serde(default, skip_serializing_if = "helper::is_0_u32")]
    pub still_frame: u32,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub axially_symmetrical: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub counterclockwise: bool,

    #[serde(
        default = "helper::f64_half",
        skip_serializing_if = "helper::is_half_f64"
    )]
    pub middle_orientation: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub orientation_range: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub apply_projection: bool,

    #[serde(flatten)]
    pub animation_params: AnimationParameters,
}

/// [`Types/RotatedAnimation`](https://lua-api.factorio.com/latest/types/RotatedAnimation.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RotatedAnimation {
    Layered {
        layers: Vec<Self>,
    },
    Striped {
        stripes: Vec<Stripe>,

        #[serde(flatten)]
        data: RotatedAnimationParameters,

        hr_version: Option<Box<Self>>,
    },
    Multi {
        filenames: Vec<FileName>,

        #[serde(flatten)]
        data: RotatedAnimationParameters,

        hr_version: Option<Box<Self>>,
    },
    Single {
        filename: FileName,

        #[serde(flatten)]
        data: RotatedAnimationParameters,

        hr_version: Option<Box<Self>>,
    },
}

/// [`Types/RotatedAnimation4Way`](https://lua-api.factorio.com/latest/types/RotatedAnimation4Way.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RotatedAnimation4Way {
    RotatedAnimation(RotatedAnimation),
    Struct {
        north: RotatedAnimation,
        east: Option<RotatedAnimation>,
        south: Option<RotatedAnimation>,
        west: Option<RotatedAnimation>,
    },
}

/// [`Types/RotatedAnimationVariations`](https://lua-api.factorio.com/latest/types/RotatedAnimationVariations.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RotatedAnimationVariations {
    Animation(RotatedAnimation),
    Array(Vec<RotatedAnimation>),
}
