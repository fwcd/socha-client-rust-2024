//! Ported from https://github.com/software-challenge/backend/blob/be88340f619892fe70c4cbd45e131d5445e883c7/plugin/src/main/kotlin/sc/plugin2024/GameState.kt

use crate::util::{Element, Error, Result};

use super::{Board, Move, Team};

/// The state of the game at a point in time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State {
    /// The game board.
    board: Board,
    /// The turn of the game.
    turn: usize,
    /// The most recent move.
    last_move: Option<Move>,
    /// The starting team.
    start_team: Team,
}

impl State {
    /// Fetches the board.
    pub fn board(&self) -> &Board { &self.board }

    /// Fetches the turn of the game.
    pub fn turn(&self) -> usize { self.turn }

    /// Fetches the most recent move.
    pub fn last_move(&self) -> Option<Move> { self.last_move }

    /// Fetches the starting team.
    pub fn start_team(&self) -> Team { self.start_team }

    /// The current team, computed from the starting team and the turn.
    pub fn current_team_from_turn(&self) -> Team {
        self.start_team.opponent_if(|_| self.turn % 2 != 0)
    }

    /// The current team.
    pub fn current_team(&self) -> Team {
        todo!()
    }

    /// Whether the game is over.
    pub fn is_over(&self) -> bool {
        todo!()
    }

    /// Fetches the winner, if any.
    pub fn winner(&self) -> Option<Team> {
        todo!()
    }

    /// Fetches the possible moves.
    pub fn possible_moves(&self) -> Vec<Move> {
        todo!()
    }

    /// Performs the given move.
    pub fn perform(&mut self, m: Move) {
        todo!()
    }

    /// Fetches the state after the given move.
    pub fn child(&self, m: Move) -> Self {
        let mut next = *self;
        next.perform(m);
        next
    }
}

impl TryFrom<&Element> for State {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        todo!()
    }
}
