use image::Rgba;
use serde::{Deserialize, Serialize};

use crate::{merge_layers, FactorioArray, GraphicsOutput, ImageCache, RenderableGraphics};

use super::{helper, Color, FileName, SpriteSizeType, Vector};

/// [`Types/IconMipMapType`](https://lua-api.factorio.com/latest/types/IconMipMapType.html)
pub type IconMipMapType = u8;

/// [`Types/IconData`](https://lua-api.factorio.com/latest/types/IconData.html)
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IconData {
    Default {
        icon: FileName,

        #[serde(flatten)]
        common: CommonIconData,
    },
    DarkBackground {
        dark_background_icon: FileName,

        #[serde(flatten)]
        common: CommonIconData,
    },
    Tintable {
        icon_tintable: FileName,

        #[serde(flatten)]
        common: CommonIconData,
    },
    TintableMask {
        icon_tintable_mask: FileName,

        #[serde(flatten)]
        common: CommonIconData,
    },
    ColorIndicatorMask {
        icon_color_indicator_mask: FileName,

        #[serde(flatten)]
        common: CommonIconData,
    },
}

impl IconData {
    #[must_use]
    pub const fn icon(&self) -> &FileName {
        match self {
            Self::Default { icon, .. } => icon,
            Self::DarkBackground {
                dark_background_icon,
                ..
            } => dark_background_icon,
            Self::Tintable { icon_tintable, .. } => icon_tintable,
            Self::TintableMask {
                icon_tintable_mask, ..
            } => icon_tintable_mask,
            Self::ColorIndicatorMask {
                icon_color_indicator_mask,
                ..
            } => icon_color_indicator_mask,
        }
    }
}

impl std::ops::Deref for IconData {
    type Target = CommonIconData;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Default { common, .. }
            | Self::DarkBackground { common, .. }
            | Self::Tintable { common, .. }
            | Self::TintableMask { common, .. }
            | Self::ColorIndicatorMask { common, .. } => common,
        }
    }
}

#[derive(Debug, Default)]
pub struct IconDataRenderOpts {
    pub icon_size: Option<SpriteSizeType>,
    pub icon_mipmaps: Option<IconMipMapType>,
}

impl RenderableGraphics for IconData {
    type RenderOpts = IconDataRenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        let ((Some(icon_size), _) | (None, Some(icon_size))) = (self.icon_size, opts.icon_size)
        else {
            return None;
        };

        let icon_size = icon_size as u32;

        // technically not 100% correct, technology icons default to 256/icon_size
        let icon_scale = self
            .scale
            .map_or_else(|| 32.0 / f64::from(icon_size), |scale| scale);

        let img = self
            .icon()
            .load(used_mods, image_cache)?
            .crop_imm(0, 0, icon_size, icon_size);

        let mut img = img.resize(
            (f64::from(img.width()) * icon_scale / scale).round() as u32,
            (f64::from(img.height()) * icon_scale / scale).round() as u32,
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

/// [`Types/IconData`](https://lua-api.factorio.com/latest/types/IconData.html)
///
/// this is needed because the fun `ItemPrototype` and `ItemWithEntityDataPrototype` think
/// its funny to change the name of a field in a different type
/// (`icon` -> `dark_background_icon` / `icon_tintable` / `icon_tintable_mask`)
#[derive(Debug, Deserialize, Serialize)]
pub struct CommonIconData {
    #[serde(
        deserialize_with = "helper::truncating_opt_deserializer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub icon_size: Option<SpriteSizeType>,

    #[serde(default = "Color::white", skip_serializing_if = "Color::is_white")]
    pub tint: Color,

    #[serde(default, skip_serializing_if = "Vector::is_0_vector")]
    pub shift: Vector,

    // TODO: Defaults to `32/icon_size` for items and recipes, `256/icon_size` for technologies.
    pub scale: Option<f64>,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub icon_mipmaps: IconMipMapType,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Icon {
    Array {
        icons: FactorioArray<IconData>,

        #[serde(
            deserialize_with = "helper::truncating_opt_deserializer",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        icon_size: Option<SpriteSizeType>,

        #[serde(
            default,
            skip_serializing_if = "helper::is_default",
            deserialize_with = "helper::truncating_deserializer"
        )]
        icon_mipmaps: IconMipMapType,
    },
    Single {
        icon: FileName,

        #[serde(deserialize_with = "helper::truncating_deserializer")]
        icon_size: SpriteSizeType,

        #[serde(
            default,
            skip_serializing_if = "helper::is_default",
            deserialize_with = "helper::truncating_deserializer"
        )]
        icon_mipmaps: IconMipMapType,
    },
}

impl RenderableGraphics for Icon {
    type RenderOpts = ();

    fn render(
        &self,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        match self {
            Self::Array {
                icons,
                icon_size,
                icon_mipmaps,
            } => merge_layers(
                icons,
                scale,
                used_mods,
                image_cache,
                &IconDataRenderOpts {
                    icon_size: *icon_size,
                    icon_mipmaps: Some(*icon_mipmaps),
                },
            ),
            Self::Single {
                icon,
                icon_size,
                icon_mipmaps,
            } => IconData::Default {
                icon: icon.clone(),
                common: CommonIconData {
                    icon_size: Some(*icon_size),
                    icon_mipmaps: *icon_mipmaps,
                    tint: Color::white(),
                    shift: Vector::default(),
                    scale: None,
                },
            }
            .render(
                scale,
                used_mods,
                image_cache,
                &IconDataRenderOpts::default(),
            ),
        }
    }
}
