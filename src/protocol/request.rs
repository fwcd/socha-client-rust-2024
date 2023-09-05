use crate::util::Element;

use super::RequestPayload;

const GAME_TYPE: &str = "swc_2024_mississippi_queen";

/// A message from the client.
#[derive(Debug, Clone)]
pub enum Request {
    /// Joins an abitrary open game.
    Join,
    /// Joins the room with the given id.
    JoinRoom { room_id: String },
    /// Joins a reserved place in a planned match with
    /// a reservation code.
    JoinPrepared { reservation_code: String },
    /// A message in a room.
    Room { room_id: String, payload: RequestPayload },
}

impl From<Request> for Element {
    fn from(req: Request) -> Self {
        match req {
            Request::Join => Element::new("join").attribute("gameType", GAME_TYPE).build(),
            Request::JoinRoom { room_id } => Element::new("joinRoom").attribute("roomId", room_id).build(),
            Request::JoinPrepared { reservation_code } => Element::new("joinPrepared").attribute("reservationCode", reservation_code).build(),
            Request::Room { room_id, payload } => Element::new("room").attribute("roomId", room_id).child(payload).build(),
        }
    }
}
