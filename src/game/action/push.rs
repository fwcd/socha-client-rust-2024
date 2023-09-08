use crate::{util::{Element, Error, Result}, game::CubeDir};

/// Nudging another ship.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Push {
    /// The direction to nudge in.
    pub direction: CubeDir,
}

impl Push {
    /// Creates a new push with the given direction.
    pub fn new(direction: CubeDir) -> Self {
        Self { direction }
    }
}

impl TryFrom<&Element> for Push {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        Ok(Self { direction: elem.attribute("direction")?.parse()? })
    }
}

impl From<Push> for Element {
    fn from(value: Push) -> Self {
        Element::new("push")
            .attribute("direction", value.direction)
            .build()
    }
}
