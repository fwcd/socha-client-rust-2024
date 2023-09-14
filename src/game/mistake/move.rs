#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MoveMistake {
    NoActions,
    PushActionRequested,
    SandbankEnd,
    FirstActionAccelerate,
    MovementPointsMissing,
}
