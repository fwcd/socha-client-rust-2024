//! Ported from https://github.com/software-challenge/backend/blob/be88340f619892fe70c4cbd45e131d5445e883c7/plugin/src/main/kotlin/sc/plugin2024/GameState.kt

use arrayvec::ArrayVec;

use crate::util::{Element, Error, Result};

use super::{Board, Move, Team, Ship, Turn, Field, CubeVec, CubeDir, Push};

/// The state of the game at a point in time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    /// The game board.
    board: Board,
    /// The turn of the game.
    turn: usize,
    /// The ships by team.
    ships: [Ship; Team::COUNT],
    /// The most recent move.
    last_move: Option<Move>,
    /// The starting team.
    start_team: Team,
    /// The team to make the next move.
    current_team: Team,
}

impl State {
    /// Fetches the board.
    pub fn board(&self) -> &Board { &self.board }

    /// Fetches the turn of the game.
    pub fn turn(&self) -> usize { self.turn }

    /// Fetches the most recent move.
    pub fn last_move(&self) -> Option<&Move> { self.last_move.as_ref() }

    /// Fetches the starting team.
    pub fn start_team(&self) -> Team { self.start_team }

    /// The next team to make a move.
    pub fn current_team(&self) -> Team { self.current_team }

    /// The opposing team.
    pub fn other_team(&self) -> Team { self.current_team.opponent() }

    /// The ship for a team.
    pub fn ship(&self, team: Team) -> Ship { self.ships[team.index()] }

    /// The current team's ship.
    pub fn current_ship(&self) -> Ship { self.ship(self.current_team()) }

    /// The opponent team's ship.
    pub fn other_ship(&self) -> Ship { self.ship(self.other_team()) }

    /// The ships.
    pub fn ships(&self) -> [Ship; Team::COUNT] { self.ships }

    /// Determines the team that should go first at the beginning of the round.
    pub fn determine_ahead_team(&self) -> Team {
        self.ships.into_iter().max_by_key(|s| (s.points, s.speed, s.coal)).unwrap().team
    }

    /// Whether the current ship must push.
    pub fn must_push(&self) -> bool {
        self.current_ship().position == self.other_ship().position
    }

    /// Fetches the possible turn actions for the current player.
    pub fn possible_turns(&self) -> Vec<Turn> {
        self.possible_turns_with(self.current_ship().coal)
    }

    /// Fetches the possible push actions for the current player.
    pub fn possible_pushes(&self) -> Vec<Push> {
        let ship = self.current_ship();
        if !self.must_push() || self.board.is_sandbank_at(ship.position) || ship.movement() < 1 {
            return Vec::new();
        }
        self.possible_pushes_at(ship.position, ship.direction)
    }

    /// Fetches the possible turn actions for the current player consuming
    /// at most the specified number of coal units.
    fn possible_turns_with(&self, max_coal: usize) -> Vec<Turn> {
        let ship = self.current_ship();
        if self.must_push() || self.board.is_sandbank_at(ship.position) {
            return Vec::new();
        }
        let max_turn_count = (max_coal + ship.free_turns).min(3) as i32;
        (1..=max_turn_count)
            .flat_map(|i| [i, -i])
            .map(|turns| Turn::new(ship.direction.rotated_by(turns)))
            .take(5)
            .collect()
    }

    /// Fetches the possible push actions at the given position
    /// with the given incoming direction.
    fn possible_pushes_at(&self, position: CubeVec, incoming_dir: CubeDir) -> Vec<Push> {
        CubeDir::ALL.into_iter()
            .filter(|&d| d != -incoming_dir && self.board.is_empty_at(position + d))
            .map(Push::new)
            .collect()
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
        let mut next = self.clone();
        next.perform(m);
        next
    }
}

impl TryFrom<&Element> for State {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        Ok(State {
            board: elem.child_by_name("board")?.try_into()?,
            turn: elem.attribute("turn")?.parse()?,
            // TODO: This currently relies on that ships are ordered by team (first ONE, then TWO)
            // We should probably parse the team attribute and then sort them instead.
            ships: elem.childs_by_name("ship")
                .map(Ship::try_from)
                .collect::<Result<ArrayVec<Ship, {Team::COUNT}>>>()?
                .into_inner()
                .map_err(|e| Error::from(format!("State has wrong number of ships: {:?}", e)))?,
            last_move: elem.child_by_name("lastMove").ok().and_then(|m| m.try_into().ok()),
            start_team: elem.attribute("startTeam")?.parse()?,
            current_team: elem.attribute("currentTeam")?.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::{game::{State, Ship, CubeVec, Team, CubeDir, Board, Segment, Field}, util::assert_xml_parse};

    #[test]
    fn test_xml_parses() {
        // TODO: Test the sub-structures too
        assert_xml_parse!(indoc! {r#"
            <state startTeam="ONE" class="state" turn="0" currentTeam="ONE">
                <board nextDirection="DOWN_RIGHT">
                    <segment direction="RIGHT">
                        <center q="0" r="0" s="0" />
                        <field-array>
                            <water />
                            <water />
                            <water />
                            <water />
                            <water />
                        </field-array>
                        <field-array>
                            <water />
                            <water />
                            <water />
                            <water />
                            <water />
                        </field-array>
                        <field-array>
                            <water />
                            <water />
                            <water />
                            <water />
                            <water />
                        </field-array>
                        <field-array>
                            <water />
                            <water />
                            <water />
                            <water />
                            <water />
                        </field-array>
                    </segment>
                    <segment direction="RIGHT">
                        <center r="0" s="-4" q="4" />
                        <field-array>
                            <water />
                            <water />
                            <water />
                            <island />
                            <water />
                        </field-array>
                        <field-array>
                            <island />
                            <water />
                            <water />
                            <water />
                            <water />
                        </field-array>
                        <field-array>
                            <passenger passenger="1" direction="DOWN_RIGHT" />
                            <water />
                            <water />
                            <water />
                            <water />
                        </field-array>
                        <field-array>
                            <water />
                            <water />
                            <water />
                            <water />
                            <water />
                        </field-array>
                    </segment>
                </board>
                <ship team="ONE" speed="1" freeTurns="1" direction="RIGHT" coal="6" passengers="0" points="0">
                    <position r="-1" q="-1" s="2" />
                </ship>
                <ship team="TWO" speed="1" coal="6" points="0" freeTurns="1" passengers="0" direction="RIGHT">
                    <position r="1" s="1" q="-2" />
                </ship>
            </state>
        "#}, State {
            board: Board {
                segments: vec![
                    Segment {
                        direction: CubeDir::Right,
                        center: CubeVec::ZERO,
                        fields: vec![vec![Field::Water; 5]; 4],
                    },
                    Segment {
                        direction: CubeDir::Right,
                        center: CubeVec::new(0, 4, -4),
                        fields: vec![
                            vec![
                                Field::Water,
                                Field::Water,
                                Field::Water,
                                Field::Island,
                                Field::Water,
                            ],
                            vec![
                                Field::Island,
                                Field::Water,
                                Field::Water,
                                Field::Water,
                                Field::Water,
                            ],
                            vec![
                                Field::Passenger { direction: CubeDir::DownRight, passenger: 1 },
                                Field::Water,
                                Field::Water,
                                Field::Water,
                                Field::Water,
                            ],
                            vec![Field::Water; 5],
                        ],
                    },
                ],
                next_direction: CubeDir::DownRight,
            },
            ships: [
                Ship {
                    team: Team::One,
                    position: CubeVec::new(-1, -1, 2),
                    direction: CubeDir::Right,
                    speed: 1,
                    free_turns: 1,
                    coal: 6,
                    passengers: 0,
                    points: 0,
                },
                Ship {
                    team: Team::Two,
                    position: CubeVec::new(1, -2, 1),
                    direction: CubeDir::Right,
                    speed: 1,
                    free_turns: 1,
                    coal: 6,
                    passengers: 0,
                    points: 0,
                },
            ],
            turn: 0,
            last_move: None,
            start_team: Team::One,
            current_team: Team::One,
        });
    }
}
