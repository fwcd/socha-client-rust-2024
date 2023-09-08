//! Ported from https://github.com/software-challenge/backend/blob/be88340f619892fe70c4cbd45e131d5445e883c7/plugin/src/main/kotlin/sc/plugin2024/Board.kt

use std::ops::{Range, Index, IndexMut};

use crate::util::{Element, Error, Result, Vec2};

use super::{CubeDir, Segment, CubeVec, Field, Ship};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub segments: Vec<Segment>,
    pub next_direction: CubeDir,
}

impl Board {
    /// The bounding box of the segments as `(min_x..(max_x + 1), min_y..(max_y + 1))`.
    pub fn bounds(&self) -> (Range<i32>, Range<i32>) {
        self.segments.iter().fold(((0..0), (0..0)), |(xs, ys), segment| {
            let center = segment.center;
            let x = center.x() / 2;
            let y = center.y();
            let xs = xs.start.min(x - 2)..xs.end.max(x + 3);
            let ys = ys.start.min(y - 2)..ys.end.max(y + 3);
            (xs, ys)
        })
    }

    /// The size of the bounding box.
    pub fn rectangle_size(&self) -> Vec2<i32> {
        let (xs, ys) = self.bounds();
        let x = xs.end - xs.start;
        let y = ys.end - ys.start;
        Vec2::new(x, y)
    }

    /// Fetches the field at the given position.
    pub fn get(&self, coords: CubeVec) -> Option<&Field> {
        self.segments.iter()
            .find(|s| (s.center - coords).length() <= 3.0)
            .and_then(|s| s.get_global(coords))
    }

    /// Fetches the field at the given position mutably.
    pub fn get_mut(&mut self, coords: CubeVec) -> Option<&mut Field> {
        self.segments.iter_mut()
            .find(|s| (s.center - coords).length() <= 3.0)
            .and_then(|s| s.get_global_mut(coords))
    }

    /// Checks whether the field has a current.
    pub fn does_field_have_current(&self, coords: CubeVec) -> bool {
        self.segment_with_index_at(coords)
            .map(|(i, s)| {
                let next_dir: CubeVec = self.segments.get(i + 1).map(|s| s.direction).unwrap_or(self.next_direction).into();
                [
                    s.center - s.direction,
                    s.center,
                    s.center + next_dir,
                    s.center + 2 * next_dir,
                ].contains(&coords)
            })
            .unwrap_or(false)
    }

    /// Fetches the distance between two fields as the number of segments.
    pub fn segment_distance(&self, coords1: CubeVec, coords2: CubeVec) -> usize {
        // TODO: Better error-handling
        let i1 = self.segment_index_at(coords1).expect("First coordinates of segment distance are out-of-bounds");
        let i2 = self.segment_index_at(coords2).expect("Second coordinates of segment distance are out-of-bounds");
        i1.abs_diff(i2)
    }

    /// Fetches a list of neighboring fields.
    pub fn neighbors(&self, coords: CubeVec) -> [Option<&Field>; 6] {
        CubeDir::ALL.map(|d| self.get(coords + d))
    }

    /// The effective speed of the given ship, depending on current.
    pub fn effective_speed(&self, ship: Ship) -> usize {
        ship.speed - (self.does_field_have_current(ship.position) as usize)
    }

    /// Fetches the segment containing the given coordinates.
    pub fn segment_at(&self, coords: CubeVec) -> Option<&Segment> {
        self.segment_with_index_at(coords).map(|(_, s)| s)
    }

    /// Fetches the index of the segment at the given coordinates.
    pub fn segment_index_at(&self, coords: CubeVec) -> Option<usize> {
        self.segment_with_index_at(coords).map(|(i, _)| i)
    }

    /// Fetches the index of the segment containing the given coordinates.
    pub fn segment_with_index_at(&self, coords: CubeVec) -> Option<(usize, &Segment)> {
        self.segments.iter().enumerate().find(|(_, s)| s.get_global(coords).is_some())
    }
}

impl Index<CubeVec> for Board {
    type Output = Field;

    fn index(&self, coords: CubeVec) -> &Self::Output {
        match self.get(coords) {
            Some(field) => field,
            None => panic!("The coordinates {} are outside the board's bounds!", coords),
        }
    }
}

impl IndexMut<CubeVec> for Board {
    fn index_mut(&mut self, coords: CubeVec) -> &mut Self::Output {
        match self.get_mut(coords) {
            Some(field) => field,
            None => panic!("The coordinates {} are outside the board's bounds!", coords),
        }
    }
}

impl TryFrom<&Element> for Board {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        Ok(Self {
            segments: elem.childs_by_name("segment")
                .map(Segment::try_from)
                .collect::<Result<Vec<Segment>>>()?,
            next_direction: elem.attribute("nextDirection")?.parse()?,
        })
    }
}
