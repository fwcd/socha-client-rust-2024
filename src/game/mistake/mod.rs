mod accelerate;
mod advance;
mod r#move;
mod push;
mod turn;

pub use accelerate::*;
pub use advance::*;
pub use r#move::*;
pub use push::*;
pub use turn::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ActionProblem {
    Accelerate(AccelerateProblem),
    Advance(AdvanceProblem),
    Push(PushProblem),
    Turn(TurnProblem),
}

impl From<AccelerateProblem> for ActionProblem {
    fn from(value: AccelerateProblem) -> Self {
        Self::Accelerate(value)
    }
}

impl From<AdvanceProblem> for ActionProblem {
    fn from(value: AdvanceProblem) -> Self {
        Self::Advance(value)
    }
}

impl From<PushProblem> for ActionProblem {
    fn from(value: PushProblem) -> Self {
        Self::Push(value)
    }
}

impl From<TurnProblem> for ActionProblem {
    fn from(value: TurnProblem) -> Self {
        Self::Turn(value)
    }
}
