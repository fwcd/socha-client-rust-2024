//! Ported from https://github.com/software-challenge/backend/blob/be88340f619892fe70c4cbd45e131d5445e883c7/plugin/src/main/kotlin/sc/plugin2024/Field.kt

use crate::util::{Error, Element, Result};

use super::Action;

/// A game move.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Move {
    pub actions: Vec<Action>,
}

impl TryFrom<&Element> for Move {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        Ok(Self {
            actions: elem.child_by_name("actions")?
                .childs()
                .map(Action::try_from)
                .collect::<Result<Vec<_>>>()?
        })
    }
}

impl From<Move> for Element {
    fn from(m: Move) -> Self {
        Element::new("data")
            .attribute("class", "move")
            .child(Element::new("actions")
                .childs(m.actions.into_iter().map(Element::from)))
            .build()
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::{util::{assert_xml_format, assert_xml_roundtrip}, game::{Move, CubeDir, Action}};

    #[test]
    fn test_to_xml() {
        assert_xml_format!(
            Move {
                actions: vec![
                    Action::Accelerate { acc: -1 },
                    Action::Turn { direction: CubeDir::DownRight },
                    Action::Advance { distance: 2 },
                ]
            },
            indoc! {r#"
                <data class="move">
                    <actions>
                        <acceleration acc="-1"/>
                        <turn direction="DOWN_RIGHT"/>
                        <advance distance="2"/>
                    </actions>
                </data>
            "#}
        );
    }

    #[test]
    fn test_xml_roundtrips() {
        assert_xml_roundtrip!(Move {
            actions: vec![
                Action::Advance { distance: 1 },
                Action::Turn { direction: CubeDir::DownRight },
                Action::Turn { direction: CubeDir::Left },
            ]
        });
    }
}
