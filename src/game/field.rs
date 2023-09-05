//! Ported from https://github.com/software-challenge/backend/blob/be88340f619892fe70c4cbd45e131d5445e883c7/plugin/src/main/kotlin/sc/plugin2024/Field.kt

use crate::util::{Error, Result, Element};

use super::CubeDir;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field {
    Water,
    Island,
    Passenger { direction: CubeDir, passenger: usize },
    Goal,
    Sandbank,
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
    use std::str::FromStr;

    use indoc::indoc;

    use crate::{game::Field, util::Element};

    #[test]
    fn test_from_xml() {
        assert_eq!(Field::try_from(&Element::from_str(indoc! {r#"
            <water />
        "#}).unwrap()).unwrap(), Field::Water);

        assert_eq!(Field::try_from(&Element::from_str(indoc! {r#"
            <island />
        "#}).unwrap()).unwrap(), Field::Island);

        assert_eq!(Field::try_from(&Element::from_str(indoc! {r#"
            <goal />
        "#}).unwrap()).unwrap(), Field::Goal);

        assert_eq!(Field::try_from(&Element::from_str(indoc! {r#"
            <sandbank />
        "#}).unwrap()).unwrap(), Field::Sandbank);
    }
}
