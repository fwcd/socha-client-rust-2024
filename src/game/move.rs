//! Ported from https://github.com/software-challenge/backend/blob/be88340f619892fe70c4cbd45e131d5445e883c7/plugin/src/main/kotlin/sc/plugin2024/Field.kt

use crate::util::{Error, Element, Result};

/// A game move.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Move;

impl TryFrom<&Element> for Move {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        todo!()
    }
}

impl From<Move> for Element {
    fn from(m: Move) -> Self {
        todo!()
    }
}
