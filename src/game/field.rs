//! Ported from https://github.com/software-challenge/backend/blob/be88340f619892fe70c4cbd45e131d5445e883c7/plugin/src/main/kotlin/sc/plugin2024/Field.kt

use crate::util::{Error, Result, Element};

use super::CubeDir;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field {
    Water,
    Island,
    Passenger { direction: CubeDir, passenger: usize },
    Goal,
}

impl TryFrom<&Element> for Field {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        todo!()
    }
}
