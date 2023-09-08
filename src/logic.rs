use log::{info, debug};

use socha_client_2024::{client::GameClientDelegate, game::{Move, Team, State}};

/// An empty game logic structure that implements the client delegate trait
/// and thus is responsible e.g. for picking a move when requested.
pub struct OwnLogic;

impl GameClientDelegate for OwnLogic {
    fn request_move(&mut self, state: &State, _my_team: Team) -> Move {
        info!("Requested move");
        let chosen_move = state.possible_moves()
            .next()
            .expect("No move found!");
        info!("Chose move {:?}", chosen_move);
        chosen_move
    }

    fn on_update_state(&mut self, state: &State) {
        debug!("Board:\n{:?}", state.board());
    }
}
