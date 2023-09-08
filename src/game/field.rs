//! Ported from https://github.com/software-challenge/backend/blob/be88340f619892fe70c4cbd45e131d5445e883c7/plugin/src/main/kotlin/sc/plugin2024/Field.kt

use crate::util::{Error, Result, Element};

use super::CubeDir;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Field {
    Water,
    Island,
    Passenger { direction: CubeDir, passenger: usize },
    Goal,
    Sandbank,
}

impl Field {
    /// Whether the field is empty.
    pub fn is_empty(self) -> bool {
        !matches!(self, Self::Island | Self::Passenger { .. })
    }
}

impl TryFrom<&Element> for Field {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        match elem.name() {
            // TODO: Check whether the serialized names are correct
            "water" => Ok(Self::Water),
            "island" => Ok(Self::Island),
            "passenger" => Ok(Self::Passenger {
                direction: elem.attribute("direction")?.parse()?,
                passenger: elem.attribute("passenger")?.parse()?,
            }),
            "goal" => Ok(Self::Goal),
            "sandbank" => Ok(Self::Sandbank),
            t => Err(Error::UnknownVariant(format!("Unknown field type: {}", t))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{util::assert_xml_parse, game::Field};

    #[test]
    fn test_xml_parses() {
        assert_xml_parse!(r#"<water />"#, Field::Water);
        assert_xml_parse!(r#"<island />"#, Field::Island);
        assert_xml_parse!(r#"<goal />"#, Field::Goal);
        assert_xml_parse!(r#"<sandbank />"#, Field::Sandbank);
    }
}
