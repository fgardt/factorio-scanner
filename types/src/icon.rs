use image::Rgba;
use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_with::skip_serializing_none;

use crate::{merge_renders, FactorioArray, GraphicsOutput, ImageCache, RenderableGraphics};

use super::{Color, FileName, RenderLayer, SpriteSizeType, Vector};

/// [`Types/IconMipMapType`](https://lua-api.factorio.com/latest/types/IconMipMapType.html)
pub type IconMipMapType = u8;

/// [`Types/IconData`](https://lua-api.factorio.com/latest/types/IconData.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct IconData {
    pub icon: FileName,

    #[serde(
        deserialize_with = "helper::truncating_deserializer",
        default = "helper::i16_64",
        skip_serializing_if = "helper::is_64_i16"
    )]
    pub icon_size: SpriteSizeType,

    #[serde(default = "Color::white", skip_serializing_if = "Color::is_white")]
    pub tint: Color,

    #[serde(default, skip_serializing_if = "Vector::is_0_vector")]
    pub shift: Vector,

    pub scale: Option<f64>,
    pub draw_background: Option<bool>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub floating: bool,
}

impl RenderableGraphics for IconData {
    type RenderOpts = ();

    fn render(
        &self,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        let icon_size = self.icon_size as u32;

        // technically not 100% correct, technology icons default to 256/icon_size
        let icon_scale = self
            .scale
            .map_or_else(|| 32.0 / f64::from(icon_size), |scale| scale);

        let img = self
            .icon
            .load(used_mods, image_cache)?
            .crop_imm(0, 0, icon_size, icon_size);

        let icon_size = f64::from(icon_size);
        let mut img = img.resize(
            (icon_size * icon_scale / scale).round() as u32,
            (icon_size * icon_scale / scale).round() as u32,
            image::imageops::FilterType::Nearest,
        );

        if !Color::is_white(&self.tint) {
            let mut img_buf = img.to_rgba8();
            let [tint_r, tint_g, tint_b, tint_a] = self.tint.to_rgba();

            for Rgba([r, g, b, a]) in img_buf.pixels_mut() {
                *r = (f64::from(*r) * tint_r).round() as u8;
                *g = (f64::from(*g) * tint_g).round() as u8;
                *b = (f64::from(*b) * tint_b).round() as u8;
                *a = (f64::from(*a) * tint_a).round() as u8;
            }
            img = img_buf.into();
        }

        Some((img, self.shift))
    }
}

macro_rules! icon_enum {
    ( $( $name:ident ),+ ) => {
        $(
            paste::paste! {
                #[derive(Debug, Deserialize, Serialize)]
                #[serde(untagged)]
                pub enum $name {
                    Single {
                        [<$name:snake>]: FileName,

                        #[serde(
                            deserialize_with = "helper::truncating_deserializer",
                            default = "helper::i16_64",
                            skip_serializing_if = "helper::is_64_i16"
                        )]
                        [<$name:snake _size>]: SpriteSizeType,
                    },
                    Layered {
                        [<$name:snake s>]: FactorioArray<IconData>,
                    },
                }

                impl RenderableGraphics for $name {
                    type RenderOpts = ();

                    fn render(
                        &self,
                        scale: f64,
                        used_mods: &mod_util::UsedMods,
                        image_cache: &mut ImageCache,
                        opts: &Self::RenderOpts,
                    ) -> Option<GraphicsOutput> {
                        match self {
                            Self::Single { [<$name:snake>], [<$name:snake _size>] } => IconData {
                                icon: [<$name:snake>].clone(),
                                icon_size: *[<$name:snake _size>],
                                tint: Color::white(),
                                shift: Vector::default(),
                                scale: None,
                                draw_background: None,
                                floating: false,
                            }
                            .render(scale, used_mods, image_cache, &()),
                            Self::Layered { [<$name:snake s>] } => merge_icon_layers([<$name:snake s>], scale, used_mods, image_cache, &()),
                        }
                    }
                }
            }
        )+
    }
}

icon_enum! {
    Icon,
    DarkBackgroundIcon,
    StarMapIcon
}

pub fn merge_icon_layers<O, T: RenderableGraphics<RenderOpts = O>>(
    layers: &[T],
    scale: f64,
    used_mods: &mod_util::UsedMods,
    image_cache: &mut ImageCache,
    opts: &O,
) -> Option<GraphicsOutput> {
    let layers = layers
        .iter()
        .filter_map(|layer| layer.render(scale, used_mods, image_cache, opts))
        .collect::<Vec<_>>();

    if layers.is_empty() {
        return None;
    }

    let (base_icon, base_shift) = &layers[0];
    let base_size = f64::from(base_icon.width());

    let layers = layers
        .iter()
        .map(|(img, shift)| {
            let shift = shift - base_shift;

            Some((img.clone(), shift / base_size))
        })
        .collect::<Vec<_>>();

    merge_renders(layers.as_slice(), scale)
}

/// [`Types/IconDrawSpecification`](https://lua-api.factorio.com/latest/types/IconDrawSpecification.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct IconDrawSpecification {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub shift: Vector,
    #[serde(default = "helper::f32_1", skip_serializing_if = "helper::is_1_f32")]
    pub scale: f32,
    #[serde(default = "helper::f32_1", skip_serializing_if = "helper::is_1_f32")]
    pub scale_for_many: f32,
    #[serde(default = "rl_eii", skip_serializing_if = "is_rl_eii")]
    pub render_layer: RenderLayer,
}

const fn rl_eii() -> RenderLayer {
    RenderLayer::EntityInfoIcon
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_rl_eii(layer: &RenderLayer) -> bool {
    *layer == rl_eii()
}

/// [`Types/IconSequencePositioning`](https://lua-api.factorio.com/latest/types/IconSequencePositioning.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct IconSequencePositioning {
    pub inventory_index: u8,
    pub max_icons_per_row: Option<u8>,
    pub max_icon_rows: Option<u8>,
    #[serde(default = "isp_shift", skip_serializing_if = "is_isp_shift")]
    pub shift: Vector,
    #[serde(default = "helper::f32_05", skip_serializing_if = "helper::is_05_f32")]
    pub scale: f32,
    #[serde(
        default = "helper::f32_1_1",
        skip_serializing_if = "helper::is_1_1_f32"
    )]
    pub separation_multiplier: f32,
    #[serde(
        default = "helper::f32_n01",
        skip_serializing_if = "helper::is_n01_f32"
    )]
    pub multi_row_initial_height_modifier: f32,
}

const fn isp_shift() -> Vector {
    Vector::Tuple(0.0, 0.7)
}

fn is_isp_shift(shift: &Vector) -> bool {
    *shift == isp_shift()
}

impl IconSequencePositioning {
    #[expect(dead_code)]
    const fn get_max_icons_per_row(&self, selection_box_width: f64) -> u8 {
        if let Some(max_icons_per_row) = self.max_icons_per_row {
            max_icons_per_row
        } else {
            (selection_box_width / 0.75) as u8
        }
    }

    #[expect(dead_code)]
    const fn get_max_icon_rows(&self, selection_box_width: f64) -> u8 {
        if let Some(max_icon_rows) = self.max_icon_rows {
            max_icon_rows
        } else {
            (selection_box_width / 1.5) as u8
        }
    }
}
