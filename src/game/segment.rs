//! Ported from https://github.com/software-challenge/backend/blob/be88340f619892fe70c4cbd45e131d5445e883c7/plugin/src/main/kotlin/sc/plugin2024/Segment.kt

use crate::util::{Error, Result, Element};

use super::{CubeDir, CubeVec, Field};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Segment {
    direction: CubeDir,
    center: CubeVec,
    fields: Vec<Vec<Field>>,
}

impl TryFrom<&Element> for Segment {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        todo!()
    }
}
