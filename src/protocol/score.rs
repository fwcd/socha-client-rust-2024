use crate::util::{Element, Error, Result};

use super::{ScoreCause};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Score {
    cause: ScoreCause,
    reason: String,
    parts: Vec<i32>,
}

impl Score {
    #[inline]
    pub fn new(cause: ScoreCause, reason: &str, parts: impl IntoIterator<Item=i32>) -> Self {
        Self { cause, reason: reason.to_owned(), parts: parts.into_iter().collect() }
    }

    #[inline]
    pub fn cause(&self) -> ScoreCause { self.cause }
    
    #[inline]
    pub fn reason(&self) -> &str { self.reason.as_str() }

    #[inline]
    pub fn parts(&self) -> &Vec<i32> { &self.parts }
}

impl TryFrom<&Element> for Score {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        Ok(Score {
            cause: elem.attribute("cause")?.parse()?,
            reason: elem.attribute("reason")?.to_owned(),
            parts: elem.childs_by_name("part").map(|p| Ok(p.content().parse::<i32>()?)).collect::<Result<_>>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use indoc::indoc;

    use crate::{util::Element, protocol::{Score, ScoreCause}};

    #[test]
    fn test_from_xml() {
        assert_eq!(Score::try_from(&Element::from_str(indoc! {r#"
            <score cause="LEFT" reason="Player left">
                <part>0</part>
                <part>15</part>
            </score>
        "#}).unwrap()).unwrap(), Score::new(ScoreCause::Left, "Player left", [0, 15]));
    }
}
