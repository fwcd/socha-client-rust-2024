use std::convert::Infallible;

pub trait UnwrapInfallible {
    type Value;

    /// Safely unwraps the value. Equivalent to `.unwrap()`, but more explicit
    /// about the fact that this method never panics.
    fn unwrap_infallible(self) -> Self::Value;
}

impl<T> UnwrapInfallible for Result<T, Infallible> {
    type Value = T;

    fn unwrap_infallible(self) -> T {
        match self {
            Ok(value) => value,
            Err(e) => match e {},
        }
    }
}
