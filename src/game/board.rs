//! Ported from https://github.com/software-challenge/backend/blob/be88340f619892fe70c4cbd45e131d5445e883c7/plugin/src/main/kotlin/sc/plugin2024/Board.kt

use std::ops::Range;

use crate::util::{Element, Error, Result, Vec2};

use super::{CubeDir, Segment, CubeVec, Field};

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
