//! Ported from https://github.com/software-challenge/backend/blob/be88340f619892fe70c4cbd45e131d5445e883c7/plugin/src/main/kotlin/sc/plugin2024/GameState.kt

use std::{collections::VecDeque, fmt, iter::once};

use arrayvec::ArrayVec;

use crate::util::{Element, Error, Result, Perform, UnwrapInfallible, Vec2};

use super::{Board, Move, Team, Ship, Turn, CubeVec, CubeDir, Push, Advance, AdvanceProblem, MAX_SPEED, Field, Accelerate, MIN_SPEED, Action, AccelerateProblem, ActionProblem, PushProblem, TurnProblem, MoveMistake, ROUND_LIMIT};

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
    #[inline]
    pub fn board(&self) -> &Board { &self.board }

    /// Fetches the turn of the game.
    #[inline]
    pub fn turn(&self) -> usize { self.turn }

    /// Fetches the most recent move.
    #[inline]
    pub fn last_move(&self) -> Option<&Move> { self.last_move.as_ref() }

    /// Fetches the starting team.
    #[inline]
    pub fn start_team(&self) -> Team { self.start_team }

    /// The next team to make a move.
    #[inline]
    pub fn current_team(&self) -> Team { self.current_team }

    /// The opposing team.
    #[inline]
    pub fn other_team(&self) -> Team { self.current_team.opponent() }

    /// The ship for a team.
    #[inline]
    pub fn ship(&self, team: Team) -> Ship { self.ships[team.index()] }

    /// The mutable ship for a team.
    #[inline]
    pub fn ship_mut(&mut self, team: Team) -> &mut Ship { &mut self.ships[team.index()] }

    /// The current team's ship.
    pub fn current_ship(&self) -> Ship { self.ship(self.current_team()) }

    /// The current team's ship, mutably.
    pub fn current_ship_mut(&mut self) -> &mut Ship { self.ship_mut(self.current_team()) }

    /// The opponent team's ship.
    pub fn other_ship(&self) -> Ship { self.ship(self.other_team()) }

    /// The opponent team's ship, mutably.
    pub fn other_ship_mut(&mut self) -> &mut Ship { self.ship_mut(self.other_team()) }

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

    /// Fetches the possible actions for the current player at the given rank in the move.
    pub fn possible_actions_at(&self, rank: i32) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::new();

        if rank == 0 {
            actions.extend(self.possible_accelerations().into_iter().map(Action::Accelerate));
        }
        actions.extend(self.possible_turns().into_iter().map(Action::Turn));
        actions.extend(self.possible_advances().into_iter().map(Action::Advance));
        if rank != 0 {
            actions.extend(self.possible_pushes().into_iter().map(Action::Push));
        }

        actions
    }

    /// Fetches the possible turn actions for the current player.
    pub fn possible_turns(&self) -> Vec<Turn> {
        self.possible_turns_with(self.current_ship().coal)
    }

    /// Fetches the possible accelerations for the current player.
    pub fn possible_accelerations(&self) -> Vec<Accelerate> {
        self.possible_accelerations_with(self.current_ship().coal)
    }

    /// Fetches the possible push actions for the current player.
    pub fn possible_pushes(&self) -> Vec<Push> {
        let ship = self.current_ship();
        if !self.must_push() || self.board.is_sandbank_at(ship.position) || ship.movement < 1 {
            return Vec::new();
        }
        self.possible_pushes_at(ship.position, ship.direction)
    }

    /// Fetches the possible advance actions for the current player.
    pub fn possible_advances(&self) -> Vec<Advance> {
        let ship = self.current_ship();
        if ship.movement < 1 || self.must_push() {
            return Vec::new();
        }
        self.possible_advances_for(ship)
    }

    /// Fetches the possible turn actions for the current player consuming
    /// at most the specified number of coal units.
    fn possible_turns_with(&self, max_coal: i32) -> Vec<Turn> {
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

    /// Fetches the possible accelerations for the current player with the given
    /// amount of coal.
    fn possible_accelerations_with(&self, max_coal: i32) -> Vec<Accelerate> {
        if self.must_push() {
            return Vec::new();
        }

        let ship = self.current_ship();
        return (1..=(max_coal + ship.free_acc))
            .flat_map(|i| [i, -i])
            .filter(|&i| if i > 0 { MAX_SPEED >= ship.speed + i } else { MIN_SPEED <= ship.speed - i })
            .map(Accelerate::new)
            .collect()
    }

    /// Fetches the possible advance actions for the given ship.
    fn possible_advances_for(&self, ship: Ship) -> Vec<Advance> {
        self.sandbank_advances_for(ship)
            .unwrap_or_else(|| self.advance_limit_for(ship).advances().collect())
    }

    /// Fetches the possible advances for a ship on a sandbank.
    fn sandbank_advances_for(&self, ship: Ship) -> Option<Vec<Advance>> {
        if self.board.is_sandbank_at(ship.position) {
            Some([-1, 1].into_iter()
                .map(Advance::new)
                .filter(|a| self.advance_limit_with(ship.position, ship.direction.opposite_if(a.distance < 0), 1).distance() > 1)
                .collect())
        } else {
            None
        }
    }

    /// Checks how far of an advancement in the given direction is possible.
    fn advance_limit_for(&self, ship: Ship) -> AdvanceLimit {
        self.advance_limit_with(ship.position, ship.direction, ship.movement)
    }

    /// Checks how far of an advancement in the given direction is possible.
    fn advance_limit_with(&self, start: CubeVec, dir: CubeDir, max_movement: i32) -> AdvanceLimit {
        let mut current_pos = start;
        let mut total_cost = 0;
        let mut has_current = false;
        let max_movement = max_movement.max(0).min(MAX_SPEED);
        let mut costs = Vec::new();

        macro_rules! result {
            ($problem:expr) => {
                AdvanceLimit { costs, problem: $problem }
            };
        }

        while total_cost < max_movement {
            current_pos += dir;
            total_cost += 1;

            if !self.board.is_empty_at(current_pos) {
                return result!(AdvanceProblem::FieldIsBlocked);
            }

            let current_field = self.board[current_pos];
            if !has_current && self.board.does_field_have_current(current_pos) {
                has_current = true;
                if total_cost < max_movement {
                    total_cost += 1;
                } else {
                    break;
                }
            }

            if self.ships.iter().any(|s| s.position == current_pos) {
                if total_cost < max_movement {
                    costs.push(total_cost);
                    return result!(AdvanceProblem::ShipAlreadyInTarget);
                }
                return result!(AdvanceProblem::InsufficientPush);
            }

            if let Field::Sandbank = current_field {
                return result!(AdvanceProblem::MoveEndOnSandbank);
            }

            costs.push(total_cost);
        }

        result!(AdvanceProblem::MovementPointsMissing { distance: None, movement: max_movement })
    }

    /// Whether the game is over.
    pub fn is_over(&self) -> bool {
        // Case 1: Ship with two passengers reaches a goal field with speed 1
        (self.turn % 2 == 0 && self.ships.into_iter().any(|s| self.is_winner(s)))
        // Case 2: Player performs an invalid move => this is handled via `MoveMistake` errors
        // Case 3: A ship is more than 3 segments behind at the end of a round
        || self.board.segment_distance(self.ships[0].position, self.ships[1].position) > 3
        // Case 4: The round limit of 30 rounds has been reached
        || self.turn / 2 >= ROUND_LIMIT
        // Case 5: Neither player can move
        || (self.last_move.is_none() && !self.can_move())
        // Otherwise the game continues...
    }

    /// Whether the given ship is a winner.
    fn is_winner(&self, ship: Ship) -> bool {
        ship.passengers > 1
        && self.board.effective_speed(ship) < 2
        && matches!(self.board.get(ship.position), Some(Field::Goal))
    }

    /// Fetches the simple moves for the current ship, falling back to the iterator of possible moves.
    pub fn sensible_moves(&self) -> Vec<Move> {
        let mut moves = self.simple_moves();
        if moves.is_empty() {
            moves.extend(self.possible_moves());
        }
        moves
    }

    /// Fetches the possible moves.
    pub fn possible_moves(&self) -> MoveIterator {
        MoveIterator::new(self.clone())
    }

    // TODO: Add sensible_moves that computes the simple moves and falls back to the iterator otherwise.

    /// Fetches the simple moves for the current ship.
    pub fn simple_moves(&self) -> Vec<Move> {
        self.simple_moves_with(self.current_ship().coal.min(1))
    }

    /// Fetches the simple moves (accelerate + turn + move) using at most the
    /// given amount of coal.
    pub fn simple_moves_with(&self, max_coal: i32) -> Vec<Move> {
        once(None)
            .chain(self.possible_turns_with(max_coal.min(1)).into_iter().map(Some))
            .flat_map(|turn| {
                let direction = turn.map(|t| t.direction).unwrap_or_else(|| self.current_ship().direction);
                let available_coal = max_coal - turn.map(|t| t.coal_cost(self.current_ship())).unwrap_or(0);
                let info = self.advance_limit_with(
                    self.current_ship().position,
                    direction,
                    self.current_ship().movement + self.current_ship().free_acc + available_coal,
                );
                let min_movement_points = (self.current_ship().movement - self.current_ship().free_acc - available_coal).max(1);
                let min_distance = info.costs.iter().position(|&c| c >= min_movement_points).map(|c| c as i32 + 1).unwrap_or(0);
                // TODO: Use either-crate + iterator to avoid boxing?
                if min_distance < 1 {
                    Vec::new()
                } else {
                    (min_distance..=info.distance())
                        .filter_map(|dist| {
                            let acc = Some(Accelerate::new(
                                info.cost_until(dist)
                                    + if dist == info.distance() && matches!(info.problem, AdvanceProblem::ShipAlreadyInTarget) { 1 } else { 0 }
                                    - self.current_ship().movement
                            )).filter(|a| a.acc != 0 && dist >= 0);

                            let push = if self.current_ship().position + (CubeVec::from(direction) * dist) == self.other_ship().position {
                                let current_rotation = self.board.segment_at(self.other_ship().position).map(|s| s.direction);
                                if let Some(p) = self.possible_pushes_at(self.other_ship().position, direction)
                                    .into_iter()
                                    .max_by_key(|p| current_rotation.map(|r| r.turn_count_to(p.direction).abs()).unwrap_or(2)) {
                                    Some(p)
                                } else {
                                    return None;
                                }
                            } else {
                                None
                            };

                            Some(Move::from_iter([
                                acc.map(Action::Accelerate),
                                turn.map(Action::Turn),
                                Some(Action::advance(dist)),
                                push.map(Action::Push),
                            ].into_iter().flatten()))
                        })
                        .collect()
                }
            })
            .collect()
    }

    /// Whether the player can move.
    pub fn can_move(&self) -> bool {
        self.possible_moves().next().is_some()
    }

    /// Increments the turn and updates the current team.
    fn advance_turn(&mut self) {
        let ship = self.current_ship_mut();
        ship.free_acc = 1;
        ship.free_turns = 1;
        ship.movement = ship.speed;

        self.turn += 1;
        self.current_team = if self.turn % 2 == 0 {
            self.determine_ahead_team()
        } else {
            self.current_team.opponent()
        };

        if !self.can_move() && !self.is_over() {
            self.last_move = None;
            self.advance_turn();
        }
    }
}

impl Perform<Accelerate> for State {
    type Error = AccelerateProblem;

    fn perform(&mut self, acc: Accelerate) -> Result<(), AccelerateProblem> {
        if acc.acc == 0 {
            return Err(AccelerateProblem::ZeroAcc);
        }

        {
            let ship = self.current_ship();

            match ship.speed {
                // TODO: Can we match against the MAX_SPEED/MIN_SPEED constants?
                7.. => return Err(AccelerateProblem::AboveMaxSpeed),
                ..=0 => return Err(AccelerateProblem::BelowMinSpeed),
                _ => {},
            }

            if self.board.is_sandbank_at(ship.position) {
                return Err(AccelerateProblem::InsufficientCoal);
            }
        }

        self.current_ship_mut().perform(acc).unwrap_infallible();

        if self.current_ship().coal < 0 {
            return Err(AccelerateProblem::InsufficientCoal);
        }

        Ok(())
    }
}

impl Perform<Advance> for State {
    type Error = AdvanceProblem;

    fn perform(&mut self, adv: Advance) -> Result<(), AdvanceProblem> {
        if (adv.distance < MIN_SPEED && !self.board.is_sandbank_at(self.current_ship().position))
            || adv.distance > MAX_SPEED {
            return Err(AdvanceProblem::InvalidDistance { distance: adv.distance });
        }
        if adv.distance > self.current_ship().movement {
            return Err(AdvanceProblem::MovementPointsMissing { distance: Some(adv.distance), movement: self.current_ship().movement });
        }

        let limit = self.advance_limit_with(
            self.current_ship().position,
            self.current_ship().direction.opposite_if(adv.distance < 0),
            self.current_ship().movement
        );

        if limit.distance() < adv.distance.abs() {
            return Err(limit.problem);
        }

        let ship = self.current_ship_mut();
        ship.position += CubeVec::from(ship.direction) * adv.distance;
        ship.movement -= limit.cost_until(adv.distance);

        Ok(())
    }
}

impl Perform<Push> for State {
    type Error = PushProblem;

    fn perform(&mut self, push: Push) -> Result<(), PushProblem> {
        let team = self.current_team();
        let nudged_team = self.other_team();

        self.ship_mut(team).movement -= 1;

        let push_from = self.ship(team).position;
        let push_to = push_from + push.direction;
        let is_sandbank = self.board.is_sandbank_at(push_to);

        let nudged_ship = self.ship_mut(nudged_team);

        if is_sandbank {
            nudged_ship.speed = 1;
            nudged_ship.movement = 1;
        }

        nudged_ship.position = push_to;
        nudged_ship.free_turns += 1;
        Ok(())
    }
}

impl Perform<Turn> for State {
    type Error = TurnProblem;

    fn perform(&mut self, turn: Turn) -> Result<(), TurnProblem> {
        let turn_count = self.current_ship().direction.turn_count_to(turn.direction);
        let abs_turn_count = turn_count.abs();
        let free_turns = self.current_ship().free_turns as i32;
        let used_coal = abs_turn_count - free_turns;

        self.current_ship_mut().free_turns = (free_turns - abs_turn_count).max(0);
        if used_coal > 0 {
            self.current_ship_mut().coal -= used_coal;
        }

        self.current_ship_mut().direction = turn.direction;
        Ok(())
    }
}

impl Perform<Action> for State {
    type Error = ActionProblem;

    /// Performs the given action.
    fn perform(&mut self, action: Action) -> Result<(), ActionProblem> {
        Ok(match action {
            Action::Accelerate(acc) => self.perform(acc)?,
            Action::Advance(adv) => self.perform(adv)?,
            Action::Push(push) => self.perform(push)?,
            Action::Turn(turn) => self.perform(turn)?,
        })
    }
}

impl Perform<Move> for State {
    type Error = MoveMistake;

    fn perform(&mut self, m: Move) -> Result<(), MoveMistake> {
        if m.is_empty() {
            return Err(MoveMistake::NoActions);
        }

        for (i, action) in m.coalesced().into_iter().enumerate() {
            if i != 0 && self.board.is_sandbank_at(self.current_ship().position) {
                return Err(MoveMistake::SandbankEnd);
            }
            if self.must_push() && !matches!(action, Action::Push(_)) {
                return Err(MoveMistake::PushActionRequired);
            }
            if i != 0 && matches!(action, Action::Accelerate(_)) {
                return Err(MoveMistake::FirstActionAccelerate);
            }
            self.perform(action)?;
        }

        match self.current_ship().movement {
            p if p > 0 => return Err(MoveMistake::MovementPointsLeft(p)),
            p if p < 0 => return Err(MoveMistake::MovementPointsMissing(-p)),
            _ => {},
        }

        // Note: We can't use `current_ship_mut()` since we need to prove to the
        // borrow checker that these mutable borrows don't overlap when mutating
        // `board`. Additionally, we need a mutable reference to each of the
        // ships, therefore we need to juggle a bit with `split_at_mut`...
        // Perhaps we should wrap the `[Ship; 2]` and then add a convenience
        // method for mutably borrowing 'our' and the other ship (given a team).
        // Maybe we'd just want to store the ships directly as fields (rather
        // than as an array)? The downside would be that deserialization would
        // be a bit less convenient (and require some manual validation)
        let (front, back) = self.ships.split_at_mut(1);
        let (ship, other_ship) = match self.current_team {
            Team::One => (&mut front[0], &mut back[0]),
            Team::Two => (&mut back[0], &mut front[0]),
        };

        self.board.pick_up_passenger(ship);
        ship.points = self.board.ship_points(*ship).expect("Could not calculate ship points");
        if m.actions.iter().any(|a| matches!(a, Action::Push(_))) {
            if other_ship.speed == 1 {
                self.board.pick_up_passenger(other_ship);
            }
            other_ship.points = self.board.ship_points(*other_ship).expect("Could not calculate other ship's points");
        }

        self.last_move = Some(m);
        self.advance_turn();

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct AdvanceLimit {
    costs: Vec<i32>,
    problem: AdvanceProblem,
}

impl AdvanceLimit {
    pub fn cost_until(&self, distance: i32) -> i32 {
        self.costs[distance as usize - 1]
    }

    pub fn distance(&self) -> i32 {
        self.costs.len() as i32
    }

    pub fn advances(&self) -> impl Iterator<Item = Advance> {
        (1..=self.distance()).rev().map(|d| Advance::new(d as i32))
    }
}

pub struct MoveIterator {
    queue: VecDeque<(State, Move)>,
}

impl MoveIterator {
    fn new(state: State) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back((state, Move::new()));
        MoveIterator { queue }
    }

    fn process(&mut self) -> Option<Move> {
        if let Some((state, current_move)) = self.queue.pop_front() {
            if !matches!(current_move.last(), Some(Action::Advance(_))) {
                for adv in state.possible_advances() {
                    let child_state = state.child(adv).unwrap();
                    let child_move = current_move.child(Action::Advance(adv)).unwrap();
                    let pushes = child_state.possible_pushes();
                    if pushes.is_empty() {
                        self.queue.push_back((child_state, child_move));
                    } else {
                        for push in pushes {
                            self.queue.push_back((
                                state.child(push).unwrap(),
                                child_move.child(Action::Push(push)).unwrap(),
                            ));
                        }
                    }
                }
            }

            if !matches!(current_move.last(), Some(Action::Turn(_))) {
                for turn in state.possible_turns() {
                    self.queue.push_back((
                        state.child(turn).unwrap(),
                        current_move.child(Action::Turn(turn)).unwrap(),
                    ));
                }
            }

            if current_move.is_empty() {
                for acc in state.possible_accelerations() {
                    let mut new_state = state.clone();
                    new_state.ships = state.ships.map(|s| {
                        if s.team == state.current_team() {
                            s.accelerated(acc.acc)
                        } else {
                            s
                        }
                    });
                    self.queue.push_back((new_state, Move::from(Action::Accelerate(acc))))
                }
            }

            Some(current_move)
        } else {
            None
        }
    }

    fn find_next(&mut self) {
        while let Some((state, _)) = self.queue.front() {
            if state.current_ship().movement == 0 {
                break;
            }
            self.process();
        }
    }
}

impl Iterator for MoveIterator {
    type Item = Move;

    fn next(&mut self) -> Option<Move> {
        self.find_next();
        self.process()
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Use better coordinate conversion
        let (xs, ys) = self.board.bounds();
        for y in ys {
            for x in xs.clone() {
                let cube_pos = CubeVec::from(Vec2::new(x, y));
                let c = self.ships
                    .into_iter()
                    .enumerate()
                    .find(|(_, s)| s.position == cube_pos)
                    .and_then(|(i, _)| char::from_digit(i as u32, 10))
                    .unwrap_or_else(|| match self.board.get(cube_pos) {
                        Some(Field::Water) => '~',
                        Some(Field::Island) => '#',
                        Some(Field::Sandbank) => '_',
                        Some(Field::Passenger { direction, passenger }) => if *passenger > 0 {
                            match direction {
                                CubeDir::Right => '>',
                                CubeDir::DownRight => '\\',
                                CubeDir::DownLeft => '/',
                                CubeDir::Left => '<',
                                CubeDir::UpLeft => '`',
                                CubeDir::UpRight => '´',
                            }
                        } else {
                            '0'
                        },
                        Some(Field::Goal) => 'G',
                        None => '.',
                    });
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
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
    use std::{fs::{DirEntry, read_to_string, create_dir_all}, str::FromStr, path::PathBuf};

    use indoc::indoc;

    use crate::{game::{State, Ship, CubeVec, Team, CubeDir, Board, Segment, Field, FREE_ACC, Move}, util::{assert_xml_parse, Element, Perform}};

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
                        center: CubeVec::new(4, 0, -4),
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
                    movement: 1,
                    free_acc: FREE_ACC,
                },
                Ship {
                    team: Team::Two,
                    position: CubeVec::new(-2, 1, 1),
                    direction: CubeDir::Right,
                    speed: 1,
                    free_turns: 1,
                    coal: 6,
                    passengers: 0,
                    points: 0,
                    movement: 1,
                    free_acc: FREE_ACC,
                },
            ],
            turn: 0,
            last_move: None,
            start_team: Team::One,
            current_team: Team::One,
        });
    }

    #[test]
    fn test_sensible_moves() {
        let manifest_dir = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).unwrap();
        let game_dir = manifest_dir.join("test-resources").join("example-game");

        create_dir_all(&game_dir).unwrap();
        let mut entries: Vec<DirEntry> = game_dir.read_dir().unwrap()
            .map(|e| e.unwrap())
            .collect();
    
        entries.sort_by_key(|e| e.file_name());

        macro_rules! assert_moves_valid {
            ($state:expr, $moves:expr $(, $args:expr)*) => {
                if let Some(ref state) = $state {
                    if let Some(ref moves) = $moves {
                        let actual = state.sensible_moves();
                        ::pretty_assertions::assert_eq!(&actual, moves $(, $args)*);
                        for m in actual {
                            state.child(m.clone()).expect(&format!("Could not perform {:?}!", m));
                        }
                    }
                }
            };
        }

        let mut state: Option<State> = None;
        let mut moves: Option<Vec<Move>> = None;
        let mut moves_path: Option<PathBuf> = None;
        let mut last_turn: Option<usize> = None;

        for entry in entries {
            let name = entry.file_name().into_string().unwrap();
            let split: Vec<_> = name.split(".").collect();
            if *split.last().unwrap() == "xml" {
                let turn: usize = split[0].parse().unwrap();
                if last_turn.map(|t| t != turn).unwrap_or(false) {
                    assert_moves_valid!(state, moves, "{:?}", &moves_path);
                    state = None;
                    moves = None;
                }
                let kind: &str = split[1];
                let element = Element::from_str(&read_to_string(entry.path()).unwrap()).unwrap();
                match kind {
                    "moves" => {
                        moves = Some(element.childs().map(|e| Move::try_from(e).unwrap()).collect());
                        moves_path = Some(entry.path().to_owned());
                    },
                    "state" => {
                        state = Some(State::try_from(&element).unwrap());
                    },
                    _ => {},
                }
                last_turn = Some(turn);
            }
        }

        assert_moves_valid!(state, moves, "{:?}", &moves_path);
    }
}
