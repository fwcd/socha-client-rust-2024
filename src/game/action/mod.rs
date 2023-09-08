//! Ported from https://github.com/software-challenge/backend/blob/be88340f619892fe70c4cbd45e131d5445e883c7/plugin/src/main/kotlin/sc/plugin2024/actions

mod accelerate;
mod advance;
mod push;
mod turn;

pub use accelerate::*;
pub use advance::*;
pub use push::*;
pub use turn::*;

use crate::util::{Error, Element, Result};

use super::CubeDir;

/// An action to take during a move.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    /// Acceleration by the given amount.
    Accelerate(Accelerate),
    /// Advancement in the direction of movement.
    Advance(Advance),
    /// Nudging another ship.
    Push(Push),
    /// A turn of the ship.
    Turn(Turn),
}

impl Action {
    /// Creates an acceleration action.
    pub fn accelerate(acc: i32) -> Self {
        Self::Accelerate(Accelerate { acc })
    }

    /// Creates an advancement action.
    pub fn advance(distance: i32) -> Self {
        Self::Advance(Advance { distance })
    }

    /// Creates a push action.
    pub fn push(direction: CubeDir) -> Self {
        Self::Push(Push { direction })
    }

    /// Creates a turn action.
    pub fn turn(direction: CubeDir) -> Self {
        Self::Turn(Turn { direction })
    }
}

impl TryFrom<&Element> for Action {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        match elem.name() {
            "acceleration" => Ok(Self::Accelerate(elem.try_into()?)),
            "advance" => Ok(Self::Advance(elem.try_into()?)),
            "push" => Ok(Self::Push(elem.try_into()?)),
            "turn" => Ok(Self::Turn(elem.try_into()?)),
            class => Err(Error::UnknownVariant(format!("Unknown move class: {}", class))),
        }
    }
}

impl From<Action> for Element {
    fn from(m: Action) -> Self {
        match m {
            Action::Accelerate(accelerate) => accelerate.into(),
            Action::Advance(advance) => advance.into(),
            Action::Push(push) => push.into(),
            Action::Turn(turn) => turn.into(),
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
            Action::accelerate(-2),
            r#"<acceleration acc="-2" />"#
        );

        assert_xml_format!(
            Action::advance(10),
            r#"<advance distance="10" />"#
        );

        assert_xml_format!(
            Action::push(CubeDir::Left),
            r#"<push direction="LEFT" />"#
        );
    }
}
