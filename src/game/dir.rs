use std::{str::FromStr, fmt};

use crate::util::{Error, Result};

/// A cube coordinate direction.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CubeDir {
    #[default]
    Right,
    DownRight,
    DownLeft,
    Left,
    UpLeft,
    UpRight,
}

impl CubeDir {
    /// Every available direction.
    pub const ALL: [Self; 6] = [
        Self::Right,
        Self::DownRight,
        Self::DownLeft,
        Self::Left,
        Self::UpLeft,
        Self::UpRight,
    ];
}

impl fmt::Display for CubeDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CubeDir::Right => write!(f, "RIGHT"),
            CubeDir::DownRight => write!(f, "DOWN_RIGHT"),
            CubeDir::DownLeft => write!(f, "DOWN_LEFT"),
            CubeDir::Left => write!(f, "LEFT"),
            CubeDir::UpLeft => write!(f, "UP_LEFT"),
            CubeDir::UpRight => write!(f, "UP_RIGHT"),
        }
    }
}

impl FromStr for CubeDir {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "RIGHT" => Ok(Self::Right),
            "DOWN_RIGHT" => Ok(Self::DownRight),
            "DOWN_LEFT" => Ok(Self::DownLeft),
            "LEFT" => Ok(Self::Left),
            "UP_LEFT" => Ok(Self::UpLeft),
            "UP_RIGHT" => Ok(Self::UpRight),
            _ => Err(Error::UnknownVariant(format!("Unknown direction {}", s))),
        }
    }
}
