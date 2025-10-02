use mod_util::UsedMods;
use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_with::skip_serializing_none;

use super::{
    Animation, Animation4Way, AnimationRenderOpts, DirectionalRenderOpts, GraphicsOutput,
    RenderLayer, RenderableGraphics, merge_renders,
};
use crate::{Color, Direction, FactorioArray, ImageCache, LightDefinition, Vector};

/// [`Types/WorkingVisualisation`](https://lua-api.factorio.com/latest/types/WorkingVisualisation.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingVisualisation {
    // TODO: get the default for this
    pub render_layer: Option<RenderLayer>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub fadeout: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub synced_fadeout: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub constant_speed: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub always_draw: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub animated_shift: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub align_to_waypoint: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub mining_drill_scorch_mark: bool,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "helper::truncating_opt_deserializer"
    )]
    pub secondary_draw_order: Option<i8>,

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

    pub north_secondary_draw_order: Option<i8>,
    pub east_secondary_draw_order: Option<i8>,
    pub south_secondary_draw_order: Option<i8>,
    pub west_secondary_draw_order: Option<i8>,

    // pub north_fog_mask: Option<FogMaskShapeDefinition>,
    // pub east_fog_mask: Option<FogMaskShapeDefinition>,
    // pub south_fog_mask: Option<FogMaskShapeDefinition>,
    // pub west_fog_mask: Option<FogMaskShapeDefinition>,
    // pub fog_mask: Option<FogMaskShapeDefinition>,
    pub draw_in_states: Option<FactorioArray<String>>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_when_state_filter_matches: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub enabled_by_name: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub name: String,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub enabled_in_animated_shift_during_waypoint_stop: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub enabled_in_animated_shift_during_transition: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub frame_based_on_shift_animation_progress: bool,

    #[serde(flatten)]
    pub scorch_mark_data: Option<WorkingVisualisationScorchMarkData>,
}

impl RenderableGraphics for WorkingVisualisation {
    type RenderOpts = DirectionalRenderOpts<AnimationRenderOpts>;

    fn render(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        self.animation
            .as_ref()?
            .render(scale, used_mods, image_cache, opts)
    }
}

/// [Types/WorkingVisualisation/Effect](https://lua-api.factorio.com/latest/types/WorkingVisualisation.html#effect)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkingVisualisationEffect {
    None,
    Flicker,
    UraniumGlow,
}

/// [Types/WorkingVisualisation/RecipeTint](https://lua-api.factorio.com/latest/types/WorkingVisualisation.html#apply_recipe_tint)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WorkingVisualisationRecipeTint {
    None,
    Primary,
    Secondary,
    Tertiary,
    Quaternary,
}

/// [Types/WorkingVisualisation/Tint](https://lua-api.factorio.com/latest/types/WorkingVisualisation.html#apply_tint)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkingVisualisationTint {
    None,
    Status,
    ResourceColor,
    InputFluidBaseColor,
    InputFluidFlowColor,
    VisualStateColor,
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
    type RenderOpts = DirectionalRenderOpts<AnimationRenderOpts>;

    fn render(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        match self {
            Self::Single { animation } => animation.render(scale, used_mods, image_cache, opts),
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
            .and_then(|a| a.render(scale, used_mods, image_cache, opts)),
        }
    }
}

/// [`Types/WorkingVisualisation/ScorchMarkData`](https://lua-api.factorio.com/latest/types/WorkingVisualisation.html#scorch_mark_fade_out_duration)
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingVisualisationScorchMarkData {
    #[serde(rename = "scorch_mark_fade_out_duration")]
    pub fade_out_duration: u16,
    #[serde(rename = "scorch_mark_lifetime")]
    pub lifetime: u16,
    #[serde(rename = "scorch_mark_fade_in_frames")]
    pub fade_in_frames: u8,
}

/// [`Types/WorkingVisualisations`](https://lua-api.factorio.com/latest/types/WorkingVisualisations.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingVisualisations<T> {
    pub animation: Option<Animation4Way>,
    pub idle_animation: Option<Animation4Way>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub always_draw_idle_animation: bool,

    pub default_recipe_tint: Option<GlobalRecipeTints>,
    pub recipe_not_set_tint: Option<GlobalRecipeTints>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub states: FactorioArray<VisualState>,

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

    #[serde(flatten)]
    child: T,
}

impl<T> std::ops::Deref for WorkingVisualisations<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T> RenderableGraphics for WorkingVisualisations<T> {
    type RenderOpts = DirectionalRenderOpts<AnimationRenderOpts>;

    fn render(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        let mut renders = vec![
            self.animation
                .as_ref()
                .and_then(|a| a.render(scale, used_mods, image_cache, opts)),
            self.idle_animation
                .as_ref()
                .and_then(|a| a.render(scale, used_mods, image_cache, opts)),
        ];

        renders.extend(
            self.working_visualisations
                .iter()
                .map(|wv| wv.render(scale, used_mods, image_cache, opts)),
        );

        merge_renders(&renders, scale)
    }
}

/// [`Types/VisualState`](https://lua-api.factorio.com/latest/types/VisualState.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct VisualState {
    pub name: String,
    pub next_active: String,
    pub next_inactive: String,
    pub duration: u8,
    pub color: Option<Color>,
}

/// [`Types/GlobalRecipeTints`](https://lua-api.factorio.com/latest/types/GlobalRecipeTints.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalRecipeTints {
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

/// [`Types/ShiftAnimationWaypoints`](https://lua-api.factorio.com/latest/types/ShiftAnimationWaypoints.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct ShiftAnimationWaypoints {
    pub north: FactorioArray<Vector>,
    pub east: FactorioArray<Vector>,
    pub south: FactorioArray<Vector>,
    pub west: FactorioArray<Vector>,
}
