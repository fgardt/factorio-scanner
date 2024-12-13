use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::SpriteParameters;
use crate::FactorioArray;

type Sprite = crate::Sprite;

/// [`Types/SpriteNWaySheet`](https://lua-api.factorio.com/latest/types/SpriteNWaySheet.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteNWaySheet<const N: u32> {
    #[serde(
        default = "SpriteNWaySheet::<N>::f_count",
        skip_serializing_if = "SpriteNWaySheet::<N>::is_f_count"
    )]
    pub frames: u32,

    #[serde(flatten)]
    parent: Box<SpriteParameters>,
}

impl<const N: u32> std::ops::Deref for SpriteNWaySheet<N> {
    type Target = SpriteParameters;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl<const N: u32> SpriteNWaySheet<N> {
    const fn f_count() -> u32 {
        N
    }

    #[expect(clippy::trivially_copy_pass_by_ref)]
    const fn is_f_count(val: &u32) -> bool {
        *val == N
    }
}

/// [`Types/Sprite4Way`](https://lua-api.factorio.com/latest/types/Sprite4Way.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[skip_serializing_none]
pub enum Sprite4Way {
    Sheets {
        sheets: FactorioArray<SpriteNWaySheet<4>>,
    },
    Sheet {
        sheet: SpriteNWaySheet<4>,
    },
    Struct {
        north: Box<Sprite>,
        east: Box<Sprite>,
        south: Box<Sprite>,
        west: Box<Sprite>,
    },
    Sprite(Sprite),
}

/// [`Types/Sprite16Way`](https://lua-api.factorio.com/latest/types/Sprite16Way.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[skip_serializing_none]
pub enum Sprite16Way {
    Sheets {
        sheets: FactorioArray<SpriteNWaySheet<16>>,
    },
    Sheet {
        sheet: SpriteNWaySheet<16>,
    },
    Struct {
        north: Box<Sprite>,
        north_north_east: Box<Sprite>,
        north_east: Box<Sprite>,
        east_north_east: Box<Sprite>,
        east: Box<Sprite>,
        east_south_east: Box<Sprite>,
        south_east: Box<Sprite>,
        south_south_east: Box<Sprite>,
        south: Box<Sprite>,
        south_south_west: Box<Sprite>,
        south_west: Box<Sprite>,
        west_south_west: Box<Sprite>,
        west: Box<Sprite>,
        west_north_west: Box<Sprite>,
        north_west: Box<Sprite>,
        north_north_west: Box<Sprite>,
    },
}
