//! Ported from https://github.com/software-challenge/backend/blob/be88340f619892fe70c4cbd45e131d5445e883c7/plugin/src/main/kotlin/sc/plugin2024/Field.kt

use crate::util::{Error, Element, Result};

use super::CubeDir;

/// A game move.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Move {
    /// Acceleration by the given amount.
    Accelerate {
        /// The amount to accelerate by. May be negative, but not zero.
        acc: i32
    },
    /// Advancement in the direction of movement.
    Advance {
        /// The number of fields to move.
        distance: usize,
    },
    /// Nudging another ship.
    Push {
        /// The direction to nudge in.
        direction: CubeDir,
    },
    /// A turn of the ship.
    Turn {
        /// The direction to turn into.
        direction: CubeDir,
    },
}

impl TryFrom<&Element> for Move {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        match elem.attribute("class")? {
            "acceleration" => Ok(Self::Accelerate { acc: elem.attribute("acc")?.parse()? }),
            "advance" => Ok(Self::Advance { distance: elem.attribute("distance")?.parse()? }),
            "push" => Ok(Self::Push { direction: elem.attribute("direction")?.parse()? }),
            "turn" => Ok(Self::Turn { direction: elem.attribute("direction")?.parse()? }),
            class => Err(Error::UnknownVariant(format!("Unknown move class: {}", class))),
        }
    }
}

impl From<Move> for Element {
    fn from(m: Move) -> Self {
        let base = Element::new("move");
        match m {
            Move::Accelerate { acc } => base
                .attribute("class", "acceleration")
                .attribute("acc", acc)
                .build(),
            Move::Advance { distance } => base
                .attribute("class", "advance")
                .attribute("distance", distance)
                .build(),
            Move::Push { direction } => base
                .attribute("class", "push")
                .attribute("direction", direction)
                .build(),
            Move::Turn { direction } => base
                .attribute("class", "turn")
                .attribute("direction", direction)
                .build(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{util::assert_xml_format, game::{Move, CubeDir}};

    // TODO: Add parse and/or roundtrip tests

    #[test]
    fn test_to_xml() {
        assert_xml_format!(
            Move::Accelerate { acc: -2 },
            r#"<move class="acceleration" acc="-2" />"#
        );

        assert_xml_format!(
            Move::Advance { distance: 10 },
            r#"<move class="advance" distance="10" />"#
        );

        assert_xml_format!(
            Move::Push { direction: CubeDir::Left },
            r#"<move class="push" direction="LEFT" />"#
        );
    }
}
