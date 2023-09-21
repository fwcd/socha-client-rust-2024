use crate::{util::{Element, Error, Result}, game::{CubeDir, Ship}};

/// A turn of the ship.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Turn {
    /// The direction to turn into.
    pub direction: CubeDir,
}

impl Turn {
    /// Creates a new turn with the given direction.
    pub fn new(direction: CubeDir) -> Self {
        Self { direction }
    }

    /// The cost in coal for the given ship.
    pub fn coal_cost(self, ship: Ship) -> i32 {
        (ship.direction.turn_count_to(self.direction).abs() - ship.free_turns).max(0)
    }
}

impl TryFrom<&Element> for Turn {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        Ok(Self { direction: elem.attribute("direction")?.parse()? })
    }
}

impl From<Turn> for Element {
    fn from(value: Turn) -> Self {
        Element::new("turn")
            .attribute("direction", value.direction)
            .build()
    }
}
