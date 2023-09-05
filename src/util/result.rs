use super::Error;

/// A shorthand notation for `Result<T, Error>`.
pub type Result<T> = std::result::Result<T, Error>;
