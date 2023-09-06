use std::collections::HashMap;

use crate::util::{Element, Error, Result};

use super::{ScoreDefinition, Player, Score};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameResult {
    definition: ScoreDefinition,
    scores: HashMap<Player, Score>,
    winner: Option<Player>,
}

impl GameResult {
    #[inline]
    pub fn new(definition: ScoreDefinition, scores: impl Into<HashMap<Player, Score>>, winner: Option<Player>) -> Self {
        Self { definition, scores: scores.into(), winner }
    }

    #[inline]
    pub fn definition(&self) -> &ScoreDefinition { &self.definition }

    #[inline]
    pub fn scores(&self) -> &HashMap<Player, Score> { &self.scores }

    #[inline]
    pub fn winner(&self) -> &Option<Player> { &self.winner }
}

impl TryFrom<&Element> for GameResult {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        Ok(Self {
            definition: elem.child_by_name("definition")?.try_into()?,
            scores: elem
                .child_by_name("scores")?
                .childs_by_name("entry")
                .map(|e| {
                    let player = Player::try_from(e.child_by_name("player")?)?;
                    let score = Score::try_from(e.child_by_name("score")?)?;
                    Ok((player, score))
                })
                .collect::<Result<_>>()?,
            winner: elem.child_by_name("winner").ok().and_then(|w| w.try_into().ok()),
        })
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::{util::assert_xml_parse, protocol::{ScoreDefinition, ScoreDefinitionFragment, ScoreAggregation, GameResult, Player, Score, ScoreCause}, game::Team, hashmap};

    #[test]
    fn test_xml_parses() {
        assert_xml_parse!(indoc! {r#"
            <data class="result">
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
                <scores>
                    <entry>
                        <player name="rad" team="ONE"/>
                        <score cause="REGULAR" reason="">
                            <part>2</part>
                            <part>27</part>
                        </score>
                    </entry>
                    <entry>
                        <player name="blues" team="TWO"/>
                        <score cause="LEFT" reason="Player left">
                            <part>0</part>
                            <part>15</part>
                        </score>
                    </entry>
                </scores>
                <winner team="ONE"/>
            </data>
        "#}, GameResult::new(
            ScoreDefinition::new([
                ScoreDefinitionFragment::new("Siegpunkte", ScoreAggregation::Sum, true),
                ScoreDefinitionFragment::new("∅ Punkte", ScoreAggregation::Average, true),
            ]),
            hashmap![
                Player::new(Some("rad"), Team::One) => Score::new(ScoreCause::Regular, "", [2, 27]),
                Player::new(Some("blues"), Team::Two) => Score::new(ScoreCause::Left, "Player left", [0, 15])
            ],
            Some(Player::new(None, Team::One))
        ));
    }
}
