use std::str::FromStr;
use std::fmt;

use crate::util::{Error, Result};

/// A playing party in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum Team {
    One = 0,
    Two = 1,
}

impl Team {
    /// The total number of teams.
    pub const COUNT: usize = 2;

    /// The teams.
    pub const ALL: [Self; Self::COUNT] = [Self::One, Self::Two];

    /// The team with the given letter.
    pub fn with_letter(letter: char) -> Option<Self> {
        match letter {
            'R' => Some(Self::One),
            'B' => Some(Self::Two),
            _ => None,
        }
    }

    /// The team's index.
    pub fn index(self) -> usize {
        self as usize
    }

    /// The opponent of the given team.
    pub fn opponent(self) -> Self {
        match self {
            Self::One => Self::Two,
            Self::Two => Self::One,
        }
    }

    /// The opponent of the given team if the given predicate is satisfied.
    pub fn opponent_if(self, predicate: impl FnOnce(Team) -> bool) -> Self {
        if predicate(self) {
            self.opponent()
        } else {
            self
        }
    }

    /// Fetches the letter associated with the team.
    pub fn letter(self) -> char {
        match self {
            Self::One => 'R',
            Self::Two => 'B',
        }
    }
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::One => write!(f, "ONE"),
            Self::Two => write!(f, "TWO"),
        }
    }
}

impl FromStr for Team {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "ONE" => Ok(Self::One),
            "TWO" => Ok(Self::Two),
            _ => Err(Error::UnknownVariant(format!("Unknown team {}", s))),
        }
    }
}
