//! Ported from https://github.com/software-challenge/backend/blob/be88340f619892fe70c4cbd45e131d5445e883c7/plugin/src/main/kotlin/sc/plugin2024/Board.kt

use crate::util::{Element, Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board;

impl TryFrom<&Element> for Board {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        todo!()
    }
}
