// TODO: Find a better name for these traits

/// A `From`-like conversion trait for wrapper types that can't implement `From`
/// generically due to overlap with the blanket `impl From<T> for T`.
pub trait From2<T> {
    /// Converts the value from another type.
    fn from2(value: T) -> Self;
}

/// An `Into`-like conversion trait for wrapper types that can't implement
/// `Into` generically due to overlap with the blanket `impl Into<T> for T`.
pub trait Into2<T> {
    /// Converts the value to another type.
    fn into2(self) -> T;
}

impl<T, U> Into2<T> for U where T: From2<U> {
    fn into2(self) -> T {
        T::from2(self)
    }
}
