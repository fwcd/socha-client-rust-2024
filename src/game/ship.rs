//! Ported from https://github.com/software-challenge/backend/blob/be88340f619892fe70c4cbd45e131d5445e883c7/plugin/src/main/kotlin/sc/plugin2024/Ship.kt

use crate::util::{Element, Error, Result};

use super::{CubeVec, MIN_SPEED, START_COAL, CubeDir, Team};

/// A player's game piece.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ship {
    pub team: Team,
    pub position: CubeVec,
    pub direction: CubeDir,
    pub speed: usize,
    pub coal: usize,
    pub passengers: usize,
    pub free_turns: usize,
    pub points: usize,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            team: Team::One,
            position: CubeVec::ZERO,
            direction: CubeDir::Right,
            speed: MIN_SPEED,
            coal: START_COAL,
            passengers: 0,
            free_turns: 1,
            points: 0,
        }
    }
}

impl Ship {
    /// The movement reach of the ship, based on the speed.
    pub fn movement(self) -> usize {
        self.speed
    }
}

impl TryFrom<&Element> for Ship {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        Ok(Self {
            team: elem.attribute("team")?.parse()?,
            position: elem.child_by_name("position")?.try_into()?,
            direction: elem.attribute("direction")?.parse()?,
            speed: elem.attribute("speed")?.parse()?,
            coal: elem.attribute("coal")?.parse()?,
            passengers: elem.attribute("passengers")?.parse()?,
            free_turns: elem.attribute("freeTurns")?.parse()?,
            points: elem.attribute("points")?.parse()?,
        })
    }
}
