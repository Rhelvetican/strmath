use crate::Str;
use std::convert::From;

impl From<&str> for Str {
    fn from(s: &str) -> Self {
        Str(s.to_string())
    }
}

impl From<String> for Str {
    fn from(s: String) -> Self {
        Str(s)
    }
}
