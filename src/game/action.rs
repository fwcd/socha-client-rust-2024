//! Ported from https://github.com/software-challenge/backend/blob/be88340f619892fe70c4cbd45e131d5445e883c7/plugin/src/main/kotlin/sc/plugin2024/actions

use crate::util::{Error, Element, Result};

use super::CubeDir;

/// An action to take during a move.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Action {
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

impl TryFrom<&Element> for Action {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        match elem.name() {
            "acceleration" => Ok(Self::Accelerate { acc: elem.attribute("acc")?.parse()? }),
            "advance" => Ok(Self::Advance { distance: elem.attribute("distance")?.parse()? }),
            "push" => Ok(Self::Push { direction: elem.attribute("direction")?.parse()? }),
            "turn" => Ok(Self::Turn { direction: elem.attribute("direction")?.parse()? }),
            class => Err(Error::UnknownVariant(format!("Unknown move class: {}", class))),
        }
    }
}

impl From<Action> for Element {
    fn from(m: Action) -> Self {
        match m {
            Action::Accelerate { acc } => Element::new("acceleration")
                .attribute("acc", acc)
                .build(),
            Action::Advance { distance } => Element::new("advance")
                .attribute("distance", distance)
                .build(),
            Action::Push { direction } => Element::new("push")
                .attribute("direction", direction)
                .build(),
            Action::Turn { direction } => Element::new("turn")
                .attribute("direction", direction)
                .build(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{util::assert_xml_format, game::{Action, CubeDir}};

    // TODO: Add parse and/or roundtrip tests

    #[test]
    fn test_xml_formats() {
        assert_xml_format!(
            Action::Accelerate { acc: -2 },
            r#"<acceleration acc="-2" />"#
        );

        assert_xml_format!(
            Action::Advance { distance: 10 },
            r#"<advance distance="10" />"#
        );

        assert_xml_format!(
            Action::Push { direction: CubeDir::Left },
            r#"<push direction="LEFT" />"#
        );
    }
}
