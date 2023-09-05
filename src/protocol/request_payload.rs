use crate::{util::Element, game::Move};

/// The data of a room message to the server.
#[derive(Debug, Clone)]
pub enum RequestPayload {
    /// A move to be performed.
    Move(Move),
}

impl From<RequestPayload> for Element {
    fn from(payload: RequestPayload) -> Self {
        match payload {
            RequestPayload::Move(m) => m.into(),
        }
    }
}
