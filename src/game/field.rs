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
        match elem.name() {
            // TODO: Check whether the serialized names are correct
            "water" => Ok(Field::Water),
            "island" => Ok(Field::Island),
            "passenger" => Ok(Field::Passenger {
                direction: elem.attribute("direction")?.parse()?,
                passenger: elem.attribute("passenger")?.parse()?,
            }),
            "goal" => Ok(Field::Goal),
            t => Err(Error::UnknownVariant(format!("Unknown field type: {}", t))),
        }
    }
}
