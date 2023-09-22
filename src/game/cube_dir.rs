use std::{str::FromStr, fmt, ops::Neg};

use crate::util::{Error, Result};

/// A cube coordinate direction.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum CubeDir {
    #[default]
    Right = 0,
    DownRight,
    DownLeft,
    Left,
    UpLeft,
    UpRight,
}

impl CubeDir {
    /// The number of directions.
    pub const COUNT: usize = 6;

    /// Every available direction.
    pub const ALL: [Self; Self::COUNT] = [
        Self::Right,
        Self::DownRight,
        Self::DownLeft,
        Self::Left,
        Self::UpLeft,
        Self::UpRight,
    ];

    /// The number of turns from the default direction (right).
    pub fn turns(self) -> i32 {
        self as i32
    }

    /// Clockwise turns to the target (in `(-2)..=3`).
    pub fn turn_count_to(self, target: Self) -> i32 {
        let diff = (target.turns() - self.turns()).rem_euclid(6);
        if diff > 3 { diff - Self::COUNT as i32 } else { diff }
    }

    /// Rotates the direction by the given number of turns.
    pub fn rotated_by(self, turns: i32) -> Self {
        Self::ALL[(self.turns() + turns).rem_euclid(Self::COUNT as i32) as usize]
    }

    /// The opposite direction if the given condition is satisfied.
    pub fn opposite_if(self, condition: bool) -> Self {
        if condition { -self } else { self }
    }
}

impl Neg for CubeDir {
    type Output = Self;

    fn neg(self) -> Self {
        Self::ALL[(self as usize + 3) % Self::COUNT]
    }
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

#[cfg(test)]
mod tests {
    use crate::game::CubeDir;

    #[test]
    fn test_turn_counts() {
        assert_eq!(CubeDir::Right.turn_count_to(CubeDir::Right), 0);
        assert_eq!(CubeDir::Right.turn_count_to(CubeDir::DownRight), 1);
        assert_eq!(CubeDir::Right.turn_count_to(CubeDir::DownLeft), 2);
        assert_eq!(CubeDir::Right.turn_count_to(CubeDir::Left), 3);
        assert_eq!(CubeDir::Right.turn_count_to(CubeDir::UpLeft), -2);
        assert_eq!(CubeDir::Right.turn_count_to(CubeDir::UpRight), -1);

        assert_eq!(CubeDir::DownRight.turn_count_to(CubeDir::Right), -1);
        assert_eq!(CubeDir::DownRight.turn_count_to(CubeDir::DownRight), 0);
        assert_eq!(CubeDir::DownRight.turn_count_to(CubeDir::DownLeft), 1);
        assert_eq!(CubeDir::DownRight.turn_count_to(CubeDir::Left), 2);
        assert_eq!(CubeDir::DownRight.turn_count_to(CubeDir::UpLeft), 3);
        assert_eq!(CubeDir::DownRight.turn_count_to(CubeDir::UpRight), -2);
    }
}
