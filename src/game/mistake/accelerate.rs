#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AccelerateProblem {
    ZeroAcc,
    AboveMaxSpeed,
    BelowMinSpeed,
    OnSandbank,
    InsufficientCoal,
}
