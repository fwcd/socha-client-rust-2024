use super::ActionProblem;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MoveMistake {
    NoActions,
    PushActionRequired,
    SandbankEnd,
    FirstActionAccelerate,
    MovementPointsLeft,
    MovementPointsMissing,
    ActionFailed(ActionProblem),
}

impl From<ActionProblem> for MoveMistake {
    fn from(value: ActionProblem) -> Self {
        MoveMistake::ActionFailed(value)
    }
}
