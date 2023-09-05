use crate::util::{Error, Result, Element};

use super::ScoreAggregation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScoreDefinitionFragment {
    name: String,
    aggregation: ScoreAggregation,
    relevant_for_ranking: bool,
}

impl ScoreDefinitionFragment {
    pub fn new(name: &str, aggregation: ScoreAggregation, relevant_for_ranking: bool) -> Self {
        Self { name: name.to_owned(), aggregation, relevant_for_ranking }
    }

    #[inline]
    pub fn name(&self) -> &str { self.name.as_str() }

    #[inline]
    pub fn aggregation(&self) -> ScoreAggregation { self.aggregation }

    #[inline]
    pub fn relevant_for_ranking(&self) -> bool { self.relevant_for_ranking }
}

impl TryFrom<&Element> for ScoreDefinitionFragment {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        Ok(ScoreDefinitionFragment {
            name: elem.attribute("name")?.to_owned(),
            aggregation: elem.child_by_name("aggregation")?.content().parse()?,
            relevant_for_ranking: elem.child_by_name("relevantForRanking")?.content().parse()?,
        })
    }
}
