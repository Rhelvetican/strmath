//! This module contains the implementations of the `Str` struct.

use crate::Str;
use std::{fmt::Display, str::Chars};

#[allow(dead_code)]
impl Str {
    /// Creates a new empty `Str`.
    pub fn new() -> Self {
        Str("".to_string())
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
    /// Returns the capacity of the `Str`.
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }
    /// Reserves additional capacity for the `Str`.
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }
    /// Reserves exact capacity for the `Str`.
    pub fn reserve_exact(&mut self, additional: usize) {
        self.0.reserve_exact(additional);
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
}
