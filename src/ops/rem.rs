//! Remainder functions

use crate::mathstr::Str;
use std::ops::{Add, Rem, RemAssign};

impl Rem<char> for Str {
    type Output = Str;
    fn rem(self, other: char) -> Self::Output {
        Str(match self.0.rfind(other) {
            Some(i) => self.0[bump(i)..].to_string(),
            None => self.0,
        })
    }
}

impl RemAssign<char> for Str {
    fn rem_assign(&mut self, other: char) {
        if let Some(i) = self.0.rfind(other) {
            self.0 = self.0[bump(i)..].to_string()
        }
    }
}

impl Rem<&str> for Str {
    type Output = Str;
    fn rem(self, other: &str) -> Self::Output {
        Str(match self.0.rfind(other) {
            Some(i) => self.0[bump(i)..].to_string(),
            None => self.0,
        })
    }
}

impl RemAssign<&str> for Str {
    fn rem_assign(&mut self, other: &str) {
        if let Some(i) = self.0.rfind(other) {
            self.0 = self.0[bump(i)..].to_string()
        }
    }
}

impl Rem<String> for Str {
    type Output = Str;
    fn rem(self, other: String) -> Self::Output {
        Str(match self.0.rfind(other.as_str()) {
            Some(i) => self.0[bump(i)..].to_string(),
            None => self.0,
        })
    }
}

impl RemAssign<String> for Str {
    fn rem_assign(&mut self, other: String) {
        if let Some(i) = self.0.rfind(other.as_str()) {
            self.0 = self.0[bump(i)..].to_string()
        }
    }
}

impl Rem<Str> for Str {
    type Output = Str;
    fn rem(self, other: Str) -> Self::Output {
        Str(match self.0.rfind(other.0.as_str()) {
            Some(i) => self.0[bump(i)..].to_string(),
            None => self.0,
        })
    }
}

impl RemAssign<Str> for Str {
    fn rem_assign(&mut self, other: Str) {
        if let Some(i) = self.0.rfind(other.0.as_str()) {
            self.0 = self.0[bump(i)..].to_string()
        }
    }
}

// Weird hacky code to bump the index.
fn bump<T>(a: T) -> T
where
    T: Add<Output = T> + From<u8>,
{
    a + T::from(1)
}
