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
    use std::str::FromStr;

    use indoc::indoc;

    use crate::{util::Element, protocol::Player, game::Team};

    #[test]
    fn test_from_xml() {
        assert_eq!(Player::try_from(&Element::from_str(indoc! {r#"
            <player name="Alice" team="ONE" />
        "#}).unwrap()).unwrap(), Player::new(Some("Alice"), Team::One));

        assert_eq!(Player::try_from(&Element::from_str(indoc! {r#"
            <player team="TWO" />
        "#}).unwrap()).unwrap(), Player::new(None, Team::Two));
    }
}
