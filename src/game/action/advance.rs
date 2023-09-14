use std::ops::{Add, AddAssign, SubAssign, Sub};

use crate::util::{Element, Error, Result};

/// Advancement in the direction of movement.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Advance {
    /// The number of fields to move.
    pub distance: i32,
}

impl Advance {
    /// Creates a new advancement with the given distance.
    pub fn new(distance: i32) -> Self {
        Self { distance }
    }
}

impl Add<Advance> for Advance {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Advance { distance: self.distance + rhs.distance }
    }
}

impl AddAssign for Advance {
    fn add_assign(&mut self, rhs: Self) {
        self.distance += rhs.distance;
    }
}

impl Sub<Advance> for Advance {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Advance { distance: self.distance - rhs.distance }
    }
}

impl SubAssign for Advance {
    fn sub_assign(&mut self, rhs: Self) {
        self.distance -= rhs.distance;
    }
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
