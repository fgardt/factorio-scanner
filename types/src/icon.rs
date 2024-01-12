use serde::{Deserialize, Serialize};

use crate::FactorioArray;

use super::{helper, Color, FileName, SpriteSizeType, Vector};

/// [`Types/IconMipMapType`](https://lua-api.factorio.com/latest/types/IconMipMapType.html)
pub type IconMipMapType = u8;

/// [`Types/IconData`](https://lua-api.factorio.com/latest/types/IconData.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct IconData {
    pub icon: FileName,

    #[serde(
        deserialize_with = "helper::truncating_opt_deserializer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub icon_size: Option<SpriteSizeType>,

    #[serde(default = "Color::white", skip_serializing_if = "Color::is_white")]
    pub tint: Color,

    #[serde(default, skip_serializing_if = "is_0_vector")]
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

#[must_use]
pub fn is_0_vector(value: &Vector) -> bool {
    value.x() == 0.0 && value.y() == 0.0
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
