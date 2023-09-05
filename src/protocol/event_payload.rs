use std::fmt;

use crate::{util::{Element, Result, Error}, game::{State, Team}};

use super::GameResult;

/// The data of a room message from the server.
#[derive(Debug, Clone)]
pub enum EventPayload {
    /// A welcome message by the server.
    Welcome(Team),
    /// A game state.
    Memento(State),
    /// A request by the server to perform a move.
    MoveRequest,
    /// A game result.
    GameResult(GameResult),
}

impl fmt::Display for EventPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Welcome(team) => write!(f, "Welcome (team: {})", team),
            Self::Memento(state) => write!(f, "Memento (turn: {})", state.turn()),
            Self::MoveRequest => write!(f, "MoveRequest"),
            Self::GameResult(result) => write!(f, "GameResult (winner: {})", result
                .winner()
                .as_ref()
                .map(|w| format!("{}", w.team()))
                .unwrap_or_else(|| "none".to_owned())),
        }
    }
}

impl TryFrom<&Element> for EventPayload {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        match elem.attribute("class")? {
            "welcomeMessage" => Ok(Self::Welcome(elem.attribute("color")?.parse()?)),
            "memento" => Ok(Self::Memento(elem.child_by_name("state")?.try_into()?)),
            "moveRequest" => Ok(Self::MoveRequest),
            "result" => Ok(Self::GameResult(elem.try_into()?)),
            "error" => Err(Error::ServerError(elem.attribute("message")?.to_owned())),
            _ => Err(Error::UnknownElement(elem.clone())),
        }
    }
}
