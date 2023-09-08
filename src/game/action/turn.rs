use crate::{util::{Element, Error, Result}, game::CubeDir};

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
