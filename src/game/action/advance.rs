use crate::util::{Element, Error, Result};

/// Advancement in the direction of movement.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Advance {
    /// The number of fields to move.
    pub distance: usize,
}

impl TryFrom<&Element> for Advance {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        Ok(Self { distance: elem.attribute("distance")?.parse()? })
    }
}

impl From<Advance> for Element {
    fn from(value: Advance) -> Self {
        Element::new("advance")
            .attribute("distance", value.distance)
            .build()
    }
}
