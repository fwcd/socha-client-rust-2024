//! Ported from https://github.com/software-challenge/backend/blob/be88340f619892fe70c4cbd45e131d5445e883c7/plugin/src/main/kotlin/sc/plugin2024/Ship.kt

use std::convert::Infallible;

use crate::util::{Element, Error, Result, Perform};

use super::{CubeVec, MIN_SPEED, START_COAL, CubeDir, Team, FREE_ACC, Accelerate};

/// A player's game piece.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ship {
    pub team: Team,
    pub position: CubeVec,
    pub direction: CubeDir,
    pub speed: i32,
    pub coal: i32,
    pub passengers: usize,
    pub free_turns: i32,
    pub points: i32,
    pub movement: i32,
    pub free_acc: i32,
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
            movement: MIN_SPEED,
            free_acc: FREE_ACC,
        }
    }
}

impl Ship {
    /// Accelerates the ship by the given amount.
    pub fn accelerate(&mut self, amount: i32) {
        self.speed += amount;
        self.movement += amount;
    }

    /// The ship after being accelerated by the given amount.
    pub fn accelerated(mut self, amount: i32) -> Self {
        self.accelerate(amount);
        self
    }
}

impl Perform<Accelerate> for Ship {
    type Error = Infallible;

    fn perform(&mut self, acc: Accelerate) -> Result<(), Infallible> {
        let used_coal = acc.acc.abs() - self.free_acc;
        if used_coal > 0 {
            self.coal -= used_coal;
            self.free_acc = 0;
        } else {
            self.free_acc = used_coal.abs();
        }
        self.accelerate(acc.acc);
        Ok(())
    }
}

impl TryFrom<&Element> for Ship {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        let speed = elem.attribute("speed")?.parse()?;
        Ok(Self {
            team: elem.attribute("team")?.parse()?,
            position: elem.child_by_name("position")?.try_into()?,
            direction: elem.attribute("direction")?.parse()?,
            speed,
            coal: elem.attribute("coal")?.parse()?,
            passengers: elem.attribute("passengers")?.parse()?,
            free_turns: elem.attribute("freeTurns")?.parse()?,
            points: elem.attribute("points")?.parse()?,
            movement: speed,
            free_acc: FREE_ACC,
        })
    }
}
