#[cfg(test)]
mod assert;
mod error;
mod result;
mod macros;
mod xml;

#[cfg(test)]
pub(crate) use assert::*;
pub use error::*;
pub use result::*;
pub use macros::*;
pub use xml::*;
