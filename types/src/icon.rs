use serde::{Deserialize, Serialize};

use crate::FactorioArray;

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
        skip_serializing_if = "helper::is_0_u8",
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
            skip_serializing_if = "helper::is_0_u8",
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
            skip_serializing_if = "helper::is_0_u8",
            deserialize_with = "helper::truncating_deserializer"
        )]
        icon_mipmaps: IconMipMapType,
    },
}
