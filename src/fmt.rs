use crate::Str;
use std::fmt::{Display, Formatter, Result};

#[cfg(feature = "display")]
impl Display for Str {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}
