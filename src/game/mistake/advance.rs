#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AdvanceProblem {
    MovementPointsMissing,
    InsufficientPush,
    InvalidDistance,
    ShipAlreadyInTarget,
    FieldIsBlocked,
    MoveEndOnSandbank,
}
