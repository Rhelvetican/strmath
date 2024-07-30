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
//!
//! ```rust
//! use strmath::MathString;
//! let s = MathString::from("Hello, ");
//! let x = MathString::from("World!");
//! let y = s + x;
//!
//! assert_eq!(y, "Hello, World!");
//! ```

mod mathstr;
mod ops;
#[cfg(feature = "serde")]
mod serde;

#[cfg(test)]
mod test;

use mathstr::Str;
pub type MathString = Str;

pub mod prelude {
    pub use crate::MathString;
}
