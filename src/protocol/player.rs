use crate::{game::Team, util::{Element, Error, Result}};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Player {
    name: Option<String>,
    team: Team,
}

impl Player {
    #[inline]
    pub fn new(name: Option<&str>, team: Team) -> Self {
        Self { name: name.map(|n| n.to_string()), team }
    }

    #[inline]
    pub fn name(&self) -> Option<&str> { self.name.as_ref().map(|n| n.as_str()) }

    #[inline]
    pub fn team(&self) -> Team { self.team }
}

impl TryFrom<&Element> for Player {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        Ok(Player {
            name: elem.attribute("name").ok().map(|s| s.to_owned()),
            team: elem.attribute("team")?.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{util::assert_xml_parse, protocol::Player, game::Team};

    #[test]
    fn test_xml_parses() {
        assert_xml_parse!(
            r#"<player name="Alice" team="ONE" />"#,
            Player::new(Some("Alice"), Team::One)
        );

        assert_xml_parse!(
            r#"<player team="TWO" />"#,
            Player::new(None, Team::Two)
        );
    }
}
