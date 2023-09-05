use std::{fmt, str::FromStr};

use crate::util::{Error, Result};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ScoreAggregation {
    Sum,
    Average,
}

impl fmt::Display for ScoreAggregation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sum => write!(f, "SUM"),
            Self::Average => write!(f, "AVERAGE"),
        }
    }
}

impl FromStr for ScoreAggregation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "SUM" => Ok(Self::Sum),
            "AVERAGE" => Ok(Self::Average),
            _ => Err(Error::UnknownVariant(format!("Unknown aggregation {}", s))),
        }
    }
}
