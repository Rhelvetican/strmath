//! This module contains the implementations of the `Str` struct.

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Deref, DerefMut},
};

use crate::impl_from;

/// A simple wrapper around a `String`.
#[derive(Debug, Clone, PartialEq, Default, Eq, Hash)]
pub struct Str(pub(crate) String);

impl Str {
    pub fn new() -> Self {
        Str(String::new())
    }
}

impl_from!(
    &str,
    String,
    &mut str,
    &String,
    ::std::borrow::Cow<'_, str>,
    ::std::boxed::Box<str>
);

impl Deref for Str {
    type Target = String;

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
