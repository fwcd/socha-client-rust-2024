#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PushProblem {
    InvalidFieldPush,
    BlockedFieldPush,
    SameFieldPush,
    SandbankPush,
    BackwardPushingRestricted,
}
