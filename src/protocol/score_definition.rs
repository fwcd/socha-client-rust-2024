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
    use indoc::indoc;

    use crate::{util::assert_xml_parse, protocol::{ScoreDefinition, ScoreDefinitionFragment, ScoreAggregation}};

    #[test]
    fn test_xml_parses() {
        assert_xml_parse!(indoc! {r#"
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
        "#}, ScoreDefinition::new([
            ScoreDefinitionFragment::new("Siegpunkte", ScoreAggregation::Sum, true),
            ScoreDefinitionFragment::new("∅ Punkte", ScoreAggregation::Average, true),
        ]));
    }
}
