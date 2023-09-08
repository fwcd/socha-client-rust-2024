use crate::util::{Element, Error, Result};

/// Acceleration by the given amount.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Accelerate {
    /// The amount to accelerate by. May be negative, but not zero.
    pub acc: i32,
}

impl Accelerate {
    /// Creates a new acceleration with the given amount to accelerate by.
    pub fn new(acc: i32) -> Self {
        Self { acc }
    }
}

impl TryFrom<&Element> for Accelerate {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        Ok(Self { acc: elem.attribute("acc")?.parse()? })
    }
}

impl From<Accelerate> for Element {
    fn from(value: Accelerate) -> Self {
        Element::new("acceleration")
            .attribute("acc", value.acc)
            .build()
    }
}
