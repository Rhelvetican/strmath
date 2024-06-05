//! Remainder functions

use crate::Str;
use std::ops::{Add, Rem, RemAssign};

/// Remains of the `Str` by a `char`.
impl Rem<char> for Str {
    type Output = Str;
    fn rem(self, other: char) -> Self::Output {
        Str(match self.0.rfind(other) {
            Some(i) => self.0[bump(i)..].to_string(),
            None => self.0,
        })
    }
}

/// Remains of the `Str` by a `char`.
impl RemAssign<char> for Str {
    fn rem_assign(&mut self, other: char) {
        if let Some(i) = self.0.rfind(other) {
            self.0 = self.0[bump(i)..].to_string()
        }
    }
}

/// Remains of the `Str` by a `&str`.
impl Rem<&str> for Str {
    type Output = Str;
    fn rem(self, other: &str) -> Self::Output {
        Str(match self.0.rfind(other) {
            Some(i) => self.0[bump(i)..].to_string(),
            None => self.0,
        })
    }
}

/// Remains of the `Str` by a `&str`.
impl RemAssign<&str> for Str {
    fn rem_assign(&mut self, other: &str) {
        if let Some(i) = self.0.rfind(other) {
            self.0 = self.0[bump(i)..].to_string()
        }
    }
}

/// Remains of the `Str` by a `String`.
impl Rem<String> for Str {
    type Output = Str;
    fn rem(self, other: String) -> Self::Output {
        Str(match self.0.rfind(other.as_str()) {
            Some(i) => self.0[bump(i)..].to_string(),
            None => self.0,
        })
    }
}

/// Remains of the `Str` by a `String`.
impl RemAssign<String> for Str {
    fn rem_assign(&mut self, other: String) {
        if let Some(i) = self.0.rfind(other.as_str()) {
            self.0 = self.0[bump(i)..].to_string()
        }
    }
}

/// Weird hacky code to bump the index.
fn bump<T>(a: T) -> T
where
    T: Add<Output = T> + From<u8>,
{
    a + T::from(1)
}
