use crate::{RealOrientation, Vector};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

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
    pub const fn is_straight(self, other: Self) -> bool {
        let a = self as u8;
        let b = other as u8;
        let b_flipped = other.flip() as u8;

        a == b || a == b_flipped
    }

    #[must_use]
    pub const fn is_right_angle(self, other: Self) -> bool {
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
    pub const fn rotate_by_direction(self, direction: Self) -> Self {
        let res = (self as u8 + direction as u8) % Self::COUNT as u8;

        match res {
            0 => Self::North,
            1 => Self::NorthNorthEast,
            2 => Self::NorthEast,
            3 => Self::EastNorthEast,
            4 => Self::East,
            5 => Self::EastSouthEast,
            6 => Self::SouthEast,
            7 => Self::SouthSouthEast,
            8 => Self::South,
            9 => Self::SouthSouthWest,
            10 => Self::SouthWest,
            11 => Self::WestSouthWest,
            12 => Self::West,
            13 => Self::WestNorthWest,
            14 => Self::NorthWest,
            15 => Self::NorthNorthWest,
            _ => unreachable!(),
        }
    }

    #[must_use]
    pub const fn to_orientation(self) -> RealOrientation {
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
    pub const fn right90(self) -> Self {
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
    pub const fn get_offset(self) -> Vector {
        const HALF_DIR_OFFSET: f64 = 0.5;

        match self {
            Self::North => Vector::new(0.0, -1.0),
            Self::NorthEast => Vector::new(1.0, -1.0),
            Self::East => Vector::new(1.0, 0.0),
            Self::SouthEast => Vector::new(1.0, 1.0),
            Self::South => Vector::new(0.0, 1.0),
            Self::SouthWest => Vector::new(-1.0, 1.0),
            Self::West => Vector::new(-1.0, 0.0),
            Self::NorthWest => Vector::new(-1.0, -1.0),
            Self::NorthNorthEast => Vector::new(HALF_DIR_OFFSET, -1.0),
            Self::EastNorthEast => Vector::new(1.0, -HALF_DIR_OFFSET),
            Self::EastSouthEast => Vector::new(1.0, HALF_DIR_OFFSET),
            Self::SouthSouthEast => Vector::new(HALF_DIR_OFFSET, 1.0),
            Self::SouthSouthWest => Vector::new(-HALF_DIR_OFFSET, 1.0),
            Self::WestSouthWest => Vector::new(-1.0, -HALF_DIR_OFFSET),
            Self::WestNorthWest => Vector::new(-1.0, HALF_DIR_OFFSET),
            Self::NorthNorthWest => Vector::new(-HALF_DIR_OFFSET, -1.0),
        }
    }

    #[must_use]
    pub const fn as_4way_idx(self) -> Option<usize> {
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

/// [`Types/DirectionString`](https://lua-api.factorio.com/latest/types/DirectionString.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DirectionString {
    North,
    NorthNorthEast,
    NorthEast,
    EastNorthEast,
    East,
    EastSouthEast,
    SouthEast,
    SouthSouthEast,
    South,
    SouthSouthWest,
    SouthWest,
    WestSouthWest,
    West,
    WestNorthWest,
    NorthWest,
    NorthNorthWest,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_straight() {
        assert!(Direction::North.is_straight(Direction::South));
        assert!(!Direction::North.is_straight(Direction::NorthNorthWest));
        assert!(!Direction::North.is_straight(Direction::East));

        assert!(Direction::EastSouthEast.is_straight(Direction::WestNorthWest));
        assert!(!Direction::EastSouthEast.is_straight(Direction::NorthWest));
        assert!(!Direction::EastSouthEast.is_straight(Direction::NorthNorthEast));
    }
}
