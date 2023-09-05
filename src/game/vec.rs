use std::{fmt, ops::{Add, Sub, Mul, Div, DivAssign, MulAssign, AddAssign, SubAssign}};

use crate::util::{Element, Error, Result};

/// A cube coordinate vector (or position).
/// (see https://www.redblobgames.com/grids/hexagons/#coordinates-cube).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CubeVec {
    r: i32,
    q: i32,
    s: i32,
}

impl Default for CubeVec {
    fn default() -> Self {
        Self::ZERO
    }
}

impl CubeVec {
    /// The coordinate origin or zero direction vector, i.e. (0, 0, 0).
    pub const ZERO: Self = Self::new(0, 0, 0);

    pub const RIGHT: Self = Self::rq(1, 0);
    pub const DOWN_RIGHT: Self = Self::rq(0, 1);
    pub const DOWN_LEFT: Self = Self::rq(-1, 1);
    pub const LEFT: Self = Self::rq(-1, 0);
    pub const UP_LEFT: Self = Self::rq(0, -1);
    pub const UP_RIGHT: Self = Self::rq(1, -1);

    // TODO: Move these to a `From<CubeDir>` implementation

    /// The unit vector for each direction.
    pub const DIRECTIONS: [Self; 6] = [
        Self::RIGHT,
        Self::DOWN_RIGHT,
        Self::DOWN_LEFT,
        Self::LEFT,
        Self::UP_LEFT,
        Self::UP_RIGHT,
    ];

    /// Creates a new vector from the given cube components.
    #[inline]
    pub const fn new(r: i32, q: i32, s: i32) -> Self {
        Self { r, q, s }
    }

    /// Creates a new vector from the given r/q components.
    #[inline]
    pub const fn rq(r: i32, q: i32) -> Self {
        Self { r, q, s: -q - r }
    }

    /// The squared length of this vector.
    #[inline]
    pub fn squared_length(self) -> i32 { self.r * self.r + self.q * self.q + self.s * self.s }

    /// The length of this vector.
    #[inline]
    pub fn length(self) -> f32 { (self.squared_length() as f32).sqrt() }

    /// The first component of this vector.
    #[inline]
    pub fn r(self) -> i32 { self.r }

    /// The second component of this vector.
    #[inline]
    pub fn q(self) -> i32 { self.q }

    /// The third component of this vector.
    #[inline]
    pub fn s(self) -> i32 { self.s }

    /// Fetches the 6 hex neighbors.
    pub fn hex_neighbors(self) -> [Self; 6] {
        Self::DIRECTIONS.map(|v| self + v)
    }
}

impl Add for CubeVec {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.r + rhs.r, self.q + rhs.q, self.s + rhs.s)
    }
}

impl AddAssign<CubeVec> for CubeVec {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.q += rhs.q;
        self.s += rhs.s;
    }
}

impl Sub for CubeVec {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.r - rhs.r, self.q - rhs.q, self.s - rhs.s)
    }
}

impl SubAssign<CubeVec> for CubeVec {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.q -= rhs.q;
        self.s -= rhs.s;
    }
}

impl Mul<i32> for CubeVec {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Self::new(self.r * rhs, self.q * rhs, self.s * rhs)
    }
}

impl Mul<CubeVec> for i32 {
    type Output = CubeVec;

    fn mul(self, rhs: CubeVec) -> CubeVec {
        CubeVec::new(self * rhs.r, self * rhs.q, self * rhs.s)
    }
}

impl MulAssign<i32> for CubeVec {
    fn mul_assign(&mut self, rhs: i32) {
        self.r *= rhs;
        self.q *= rhs;
        self.s *= rhs;
    }
}

impl Div<i32> for CubeVec {
    type Output = Self;

    fn div(self, rhs: i32) -> Self {
        Self::new(self.r / rhs, self.q / rhs, self.s / rhs)
    }
}

impl DivAssign<i32> for CubeVec {
    fn div_assign(&mut self, rhs: i32) {
        self.r /= rhs;
        self.q /= rhs;
        self.s /= rhs;
    }
}

impl fmt::Display for CubeVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.q, self.s)
    }
}

impl TryFrom<&Element> for CubeVec {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        Ok(CubeVec::new(
            elem.attribute("r")?.parse()?,
            elem.attribute("q")?.parse()?,
            elem.attribute("s")?.parse()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use indoc::indoc;

    use crate::{util::Element, game::CubeVec};

    #[test]
    fn test_from_xml() {
        assert_eq!(CubeVec::try_from(&Element::from_str(indoc! {r#"
            <position r="23" q="0" s="-2" />
        "#}).unwrap()).unwrap(), CubeVec::new(23, 0, -2));
    }
}
