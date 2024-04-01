//! # fmt
//! The module that holds the formatting functions.

use crate::Str;
use std::fmt::{Display, Formatter, Result};

#[cfg(feature = "display")]
impl Display for Str {
    /// Formats the `Str` struct to a string with a specified formatter.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}
