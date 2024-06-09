//! A simple library for weird string manipulation techniques.
//!
//! Strmath is a library that provides a simple way to manipulate strings in a weird way. Such as:
//! * Adding a string to another string or anything that implements the trait Display.
//! * Subtracting a string from another string.
//! * Multiplying a string with another string/chars.
//! * Dividing a string with another string/chars.
//!
//! # Glossary
//! * Str: The main struct that holds the string.
//! * Ops: The module that holds the operations.
//!
//! [fmt]: crate::fmt
//! [imp]: crate::imp
//! [ops]: crate::ops
//!
//! ## Str
//! The main struct that holds the string.
//! It is *just* a simple wrapper around the String struct.
//! ```rust
//! use strmath::Str;
//! let s = Str::from("Hello, ");
//! let x = Str::from("World!");
//! let y = s + x;
//! ```
//!
//! ## Ops
//! The module that holds the operations. Refer to the [_ops_][ops] module for more information.
//! It contains the following operations:
//! * Add
//! * AddAssign
//! * Sub
//! * SubAssign
//! * Mul
//! * MulAssign
//! * Div
//! * DivAssign
//! * Rem
//! * RemAssign
//!
//! ## Implementations
//! The module that holds the implementations. Refer to the [_imp_][imp] module for more information.
//!
//! ## Formatting
//! The module that holds the formatting functions. Refer to the [_fmt_][fmt] module for more information.

use std::string::String as StdString;

mod fmt;
mod r#impl;
mod ops;

#[cfg(test)]
mod test;

#[derive(Debug, Clone, PartialEq, Default, Eq, Hash)]
pub struct Str(StdString);

pub type String = Str;

pub mod prelude {
    pub use crate::String;
}
