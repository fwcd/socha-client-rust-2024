use std::{fmt, ops::{Add, Sub, Mul, Div, DivAssign, MulAssign}, marker::PhantomData};

use crate::util::{Element, Error, Result};

/// Marker type for direct coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direct {}

/// Marker type for doubled coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Doubled {}

/// A position on the board or 2D integer vector.
/// Either uses direct or doubled hex coordinates.
/// (see https://www.redblobgames.com/grids/hexagons/#coordinates-doubled).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2<C> where C: Copy {
    pub x: i32,
    pub y: i32,
    phantom: PhantomData<C>,
}

impl<C> Default for Vec2<C> where C: Copy {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<C> Vec2<C> where C: Copy {
    /// The coordinate origin, i.e. (0, 0).
    pub const ZERO: Self = Self::new(0, 0);

    /// Creates a new vector from the given x- and y-components.
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y, phantom: PhantomData }
    }

    /// The area of the rectangle spanned by this vector.
    #[inline]
    pub fn area(self) -> i32 { (self.x * self.y).abs() }

    /// The squared length of this vector.
    #[inline]
    pub fn squared_length(self) -> i32 { self.x * self.x + self.y * self.y }

    /// The length of this vector.
    #[inline]
    pub fn length(self) -> f32 { (self.squared_length() as f32).sqrt() }
}

impl Vec2<Direct> {
    /// The four vectors in cardinal direction.
    pub const CARDINALS: [Self; 4] = [
        Self::new(-1,  0),
        Self::new( 1,  0),
        Self::new( 0, -1),
        Self::new( 0,  1),
    ];

    /// The four vectors in diagonal direction.
    pub const DIAGONALS: [Self; 4] = [
        Self::new(-1, -1),
        Self::new(-1,  1),
        Self::new( 1, -1),
        Self::new( 1,  1),
    ];

    /// Fetches the cardinal neighbors.
    pub fn cardinal_neighbors(self) -> [Self; 4] {
        Self::CARDINALS.map(|v| self + v)
    }

    /// A convenience method for converting to doubled coordinates.
    pub fn to_doubled(self) -> Vec2<Doubled> {
        self.into()
    }
}

impl Vec2<Doubled> {
    pub const LEFT: Self = Self::new(2, 0);
    pub const RIGHT: Self = Self::new(-2, 0);
    pub const UP_LEFT: Self = Self::new(-1, -1);
    pub const UP_RIGHT: Self = Self::new(1, -1);
    pub const DOWN_LEFT: Self = Self::new(-1, 1);
    pub const DOWN_RIGHT: Self = Self::new(1, 1);

    /// The directions on the hex board.
    pub const DIRECTIONS: [Self; 6] = [
        Self::LEFT,
        Self::UP_LEFT,
        Self::UP_RIGHT,
        Self::RIGHT,
        Self::DOWN_RIGHT,
        Self::DOWN_LEFT,
    ];

    /// Whether the vector is parallel to a hex axis.
    pub fn straight(self) -> bool {
        self.x.abs() == self.y.abs() || (self.x % 2 == 0 && self.y == 0)
    }

    /// Fetches the 6 hex neighbors.
    pub fn hex_neighbors(self) -> [Self; 6] {
        Self::DIRECTIONS.map(|v| self + v)
    }

    /// A convenience method for converting to direct coordinates.
    pub fn to_direct(self) -> Vec2<Direct> {
        self.into()
    }
}

impl From<Vec2<Doubled>> for Vec2<Direct> {
    /// Converts this vector from doubled hex coordinates.
    fn from(v: Vec2<Doubled>) -> Self {
        Self::new(v.x / 2, v.y)
    }
}

impl From<Vec2<Direct>> for Vec2<Doubled> {
    /// Converts this vector from doubled hex coordinates.
    fn from(v: Vec2<Direct>) -> Self {
        Self::new(v.x * 2 + v.y % 2, v.y)
    }
}

impl<C> Add for Vec2<C> where C: Copy {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<C> Sub for Vec2<C> where C: Copy {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<C> Mul<i32> for Vec2<C> where C: Copy {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl<C> Mul<Vec2<C>> for i32 where C: Copy {
    type Output = Vec2<C>;

    fn mul(self, rhs: Vec2<C>) -> Vec2<C> {
        Vec2::new(self * rhs.x, self * rhs.y)
    }
}

impl<C> MulAssign<i32> for Vec2<C> where C: Copy {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<C> Div<i32> for Vec2<C> where C: Copy {
    type Output = Self;

    fn div(self, rhs: i32) -> Self {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl<C> DivAssign<i32> for Vec2<C> where C: Copy {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<C> fmt::Display for Vec2<C> where C: Copy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<C> TryFrom<&Element> for Vec2<C> where C: Copy {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        Ok(Vec2::new(elem.attribute("x")?.parse()?, elem.attribute("y")?.parse()?))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use indoc::indoc;

    use crate::{util::Element, game::{Vec2, Direct}};

    #[test]
    fn test_from_xml() {
        assert_eq!(Vec2::<Direct>::try_from(&Element::from_str(indoc! {r#"
            <coords x="23" y="0" />
        "#}).unwrap()).unwrap(), Vec2::new(23, 0));
    }
}
