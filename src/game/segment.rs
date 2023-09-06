//! Ported from https://github.com/software-challenge/backend/blob/be88340f619892fe70c4cbd45e131d5445e883c7/plugin/src/main/kotlin/sc/plugin2024/Segment.kt

use crate::util::{Error, Result, Element};

use super::{CubeDir, CubeVec, Field};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Segment {
    pub direction: CubeDir,
    pub center: CubeVec,
    pub fields: Vec<Vec<Field>>,
}

impl TryFrom<&Element> for Segment {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        Ok(Self {
            direction: elem.attribute("direction")?.parse()?,
            center: elem.child_by_name("center")?.try_into()?,
            fields: elem.childs_by_name("field-array")
                .map(|fa| fa.childs()
                    .map(Field::try_from)
                    .collect::<Result<Vec<_>>>())
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::{game::{Segment, CubeDir, CubeVec, Field}, util::assert_xml_parse};

    #[test]
    fn test_xml_parses() {
        assert_xml_parse!(indoc! {r#"
            <segment direction="RIGHT">
                <center q="0" r="0" s="0" />
                <field-array>
                    <water />
                    <water />
                    <water />
                    <water />
                    <water />
                </field-array>
                <field-array>
                    <water />
                    <water />
                    <water />
                    <water />
                    <water />
                </field-array>
                <field-array>
                    <water />
                    <water />
                    <water />
                    <water />
                    <water />
                </field-array>
                <field-array>
                    <water />
                    <water />
                    <water />
                    <water />
                    <water />
                </field-array>
            </segment>
        "#}, Segment {
            direction: CubeDir::Right,
            center: CubeVec::ZERO,
            fields: vec![vec![Field::Water; 5]; 4],
        });
    }
}
