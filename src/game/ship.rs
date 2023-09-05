//! Ported from https://github.com/software-challenge/backend/blob/be88340f619892fe70c4cbd45e131d5445e883c7/plugin/src/main/kotlin/sc/plugin2024/Ship.kt

use crate::util::{Element, Error, Result};

use super::{CubeVec, MIN_SPEED, START_COAL};

/// A player's game piece.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ship {
    pub position: CubeVec,
    pub direction: CubeVec,
    pub speed: usize,
    pub coal: usize,
    pub passengers: usize,
    pub free_turns: usize,
    pub points: usize,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            position: CubeVec::ZERO,
            direction: CubeVec::RIGHT,
            speed: MIN_SPEED,
            coal: START_COAL,
            passengers: 0,
            free_turns: 1,
            points: 0,
        }
    }
}

impl TryFrom<&Element> for Ship {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        todo!()
    }
}
