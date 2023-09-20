#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AdvanceProblem {
    MovementPointsMissing { distance: Option<i32>, movement: i32 },
    InsufficientPush,
    InvalidDistance { distance: i32 },
    ShipAlreadyInTarget,
    FieldIsBlocked,
    MoveEndOnSandbank,
}
