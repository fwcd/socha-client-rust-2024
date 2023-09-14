//! Ported from https://github.com/software-challenge/backend/blob/be88340f619892fe70c4cbd45e131d5445e883c7/plugin/src/main/kotlin/sc/plugin2024/util/Constants.kt

pub const ROUND_LIMIT: usize = 30;
    
// Ship Properties
pub const START_COAL: i32 = 6;
pub const MIN_SPEED: i32 = 1;
pub const MAX_SPEED: i32 = 6;
pub const FREE_ACC: i32 = 1;

// Points
pub const FINISH_POINTS: usize = 6;

pub const POINTS_PER_PASSENGER: i32 = 5;
pub const POINTS_PER_SEGMENT: i32 = 5;

pub const NUMBER_OF_PASSENGERS: usize = 5;

// Board
pub const SEGMENT_FIELDS_WIDTH: usize = 4;
pub const SEGMENT_FIELDS_HEIGHT: usize = 5;
pub const NUMBER_OF_SEGMENTS: usize = 8;

// Board Fields
pub const MAX_SPECIAL: usize = 0; // Sandbanks disabled
pub const MIN_SPECIAL: usize = 0;
pub const MAX_ISLANDS: usize = 3;
pub const MIN_ISLANDS: usize = 2;
