use crate::util::{Element, Error, Result};

use super::ScoreDefinitionFragment;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScoreDefinition {
    fragments: Vec<ScoreDefinitionFragment>,
}

impl ScoreDefinition {
    pub fn new(fragments: impl IntoIterator<Item=ScoreDefinitionFragment>) -> Self {
        Self { fragments: fragments.into_iter().collect() }
    }

    #[inline]
    pub fn fragments(&self) -> &Vec<ScoreDefinitionFragment> { &self.fragments }
}

impl TryFrom<&Element> for ScoreDefinition {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        Ok(ScoreDefinition {
            fragments: elem.childs_by_name("fragment").map(ScoreDefinitionFragment::try_from).collect::<Result<_>>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use indoc::indoc;

    use crate::{util::Element, protocol::{ScoreDefinition, ScoreDefinitionFragment, ScoreAggregation}};

    #[test]
    fn test_from_xml() {
        assert_eq!(ScoreDefinition::try_from(&Element::from_str(indoc! {r#"
            <definition>
                <fragment name="Siegpunkte">
                    <aggregation>SUM</aggregation>
                    <relevantForRanking>true</relevantForRanking>
                </fragment>
                <fragment name="∅ Punkte">
                    <aggregation>AVERAGE</aggregation>
                    <relevantForRanking>true</relevantForRanking>
                </fragment>
            </definition>
        "#}).unwrap()).unwrap(), ScoreDefinition::new([
            ScoreDefinitionFragment::new("Siegpunkte", ScoreAggregation::Sum, true),
            ScoreDefinitionFragment::new("∅ Punkte", ScoreAggregation::Average, true),
        ]));
    }
}
