use super::Error;

/// A shorthand notation for `Result<T, Error>`.
pub type Result<T, E = Error> = std::result::Result<T, E>;
