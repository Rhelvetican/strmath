use crate::Str;
use std::{fmt::Display, str::Chars};

#[allow(dead_code)]
impl Str {
    pub fn new() -> Self {
        Str("".to_string())
    }
    pub fn from<T: Display>(s: T) -> Self {
        Str(s.to_string())
    }
    pub fn to_str(&self) -> &str {
        &self.0
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Str(String::with_capacity(capacity))
    }
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }
    pub fn reserve_exact(&mut self, additional: usize) {
        self.0.reserve_exact(additional);
    }
    pub fn reverse(&mut self) {
        self.0 = self.0.chars().rev().collect();
    }
    pub fn push(&mut self, s: &str) {
        self.0 += s;
    }
    pub fn chars(&self) -> Chars<'_> {
        self.0.chars()
    }
}
