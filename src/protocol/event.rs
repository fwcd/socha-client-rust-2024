use crate::util::{Element, Result, Error};

use super::EventPayload;

/// A message from the server.
#[derive(Debug)]
pub enum Event {
    /// Notifies the client that they successfully joined a room.
    Joined { room_id: String },
    /// Notifies the client that they left a room.
    Left { room_id: String },
    /// A message in a room.
    Room { room_id: String, payload: EventPayload },
}

impl TryFrom<&Element> for Event {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        match elem.name() {
            "joined" => Ok(Self::Joined { room_id: elem.attribute("roomId")?.to_owned() }),
            "left" => Ok(Self::Left { room_id: elem.attribute("roomId")?.to_owned() }),
            "room" => Ok(Self::Room {
                room_id: elem.attribute("roomId")?.to_owned(),
                payload: elem.child_by_name("data")?.try_into()?,
            }),
            _ => Err(Error::UnknownElement(elem.clone())),
        }
    }
}
