//! Ported from https://github.com/software-challenge/backend/blob/be88340f619892fe70c4cbd45e131d5445e883c7/plugin/src/main/kotlin/sc/plugin2024/Segment.kt

use std::ops::{Index, IndexMut};

use crate::util::{Error, Result, Element};

use super::{CubeDir, CubeVec, Field};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Segment {
    pub direction: CubeDir,
    pub center: CubeVec,
    pub fields: Vec<Vec<Field>>,
}

impl Segment {
    /// The position of the segment's tip.
    pub fn tip(&self) -> CubeVec {
        self.center + CubeVec::from(self.direction) * (self.fields.len() as i32 / 2)
    }

    /// Converts local to global coordinates.
    pub fn local_to_global(&self, coords: CubeVec) -> CubeVec {
        coords.rotated_by(CubeDir::Right.turn_count_to(self.direction)) + self.center
    }

    /// Converts global to local coordinates.
    pub fn global_to_local(&self, coords: CubeVec) -> CubeVec {
        (coords - self.center).rotated_by(self.direction.turn_count_to(CubeDir::Right))
    }

    /// Fetches the field at the given global position.
    pub fn get_global(&self, coords: CubeVec) -> Option<&Field> {
        self.get_local(self.global_to_local(coords))
    }

    /// Fetches the field at the given global position mutably.
    pub fn get_global_mut(&mut self, coords: CubeVec) -> Option<&mut Field> {
        self.get_local_mut(self.global_to_local(coords))
    }

    /// Fetches the field at the given local position.
    pub fn get_local(&self, coords: CubeVec) -> Option<&Field> {
        let (x, y) = Self::array_coords(coords);
        self.fields.get(x).and_then(|c| c.get(y))
    }

    /// Fetches the field at the given local position mutably.
    pub fn get_local_mut(&mut self, coords: CubeVec) -> Option<&mut Field> {
        let (x, y) = Self::array_coords(coords);
        self.fields.get_mut(x).and_then(|c| c.get_mut(y))
    }

    /// Fetches the array indices for the given local position.
    fn array_coords(coords: CubeVec) -> (usize, usize) {
        ((coords.array_x() + 1) as usize, (coords.r() + 2) as usize)
    }
}

impl Index<CubeVec> for Segment {
    type Output = Field;

    fn index(&self, coords: CubeVec) -> &Self::Output {
        match self.get_global(coords) {
            Some(field) => field,
            None => panic!("The global coordinates {} (locally: {}) are outside the segment's bounds!", coords, self.global_to_local(coords)),
        }
    }
}

impl IndexMut<CubeVec> for Segment {
    fn index_mut(&mut self, coords: CubeVec) -> &mut Self::Output {
        // For some reason the borrow checker is overly conservative with the
        // mutable borrow and doesn't let us compute the global coords lazily. See
        // - https://www.reddit.com/r/rust/comments/11kih5s/returning_a_borrow_from_an_iflet_with_a_mutable/
        // - https://github.com/rust-lang/rust/issues/54663
        let global_coords = self.global_to_local(coords);
        match self.get_global_mut(coords) {
            Some(field) => field,
            None => panic!("The global coordinates {} (locally: {}) are outside the segment's bounds!", coords, global_coords),
        }
    }
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
