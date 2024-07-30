//! This module contains the implementations of the `Str` struct.

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Deref, DerefMut},
    str::Chars,
    string::{FromUtf8Error, String as StdString},
};

/// A simple wrapper around a `String`.
#[derive(Debug, Clone, PartialEq, Default, Eq, Hash)]
pub struct Str(pub(crate) StdString);

impl Str {
    /// Creates a new empty `Str`.
    pub fn new() -> Self {
        Str(String::new())
    }

    /// Creates a new `Str` from any types that implements the `Display` trait.
    pub fn from<T: Display>(s: T) -> Self {
        Str(s.to_string())
    }

    /// Converts the `Str` struct to a `&str`.
    pub fn to_str(&self) -> &str {
        &self.0
    }

    /// Creates a new empty `Str` with a specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Str(String::with_capacity(capacity))
    }

    /// Reverse the `String` contained inside the `Str`.
    pub fn reverse(&mut self) {
        self.0 = self.0.chars().rev().collect();
    }

    /// Pushes a string slice into the `Str`.
    pub fn push(&mut self, s: &str) {
        self.0 += s;
    }

    /// Returns an iterator over the characters of the `Str`.
    pub fn chars(&self) -> Chars<'_> {
        self.0.chars()
    }

    /// Returns the length of the `Str`.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the `Str` is empty. Otherwise, returns `false`.
    /// This is equivalent to `self.len() == 0`.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Truncates the `Str` to the specified length.
    /// If `new_len` is greater than the current length, this has no effect.
    pub fn truncate(&mut self, new_len: usize) {
        self.0.truncate(new_len);
    }

    /// Removes the last character from the `Str` and returns it.
    /// Returns `None` if the `Str` is empty.
    pub fn pop(&mut self) -> Option<char> {
        self.0.pop()
    }

    /// Truncates this `Str`, removing all contents.
    /// While this means the `Str` will have a length of zero, it does not touch its capacity.
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Converts a vector of bytes to a `Str`.
    pub fn from_utf8(v: Vec<u8>) -> Result<Self, FromUtf8Error> {
        match String::from_utf8(v) {
            Ok(s) => Ok(Str(s)),
            Err(e) => Err(e),
        }
    }

    /// Converts the `Str` to a `String`.
    pub fn into_string(self) -> StdString {
        self.0
    }
}

impl AsRef<str> for Str {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<[u8]> for Str {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl Deref for Str {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Str {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Str {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}
