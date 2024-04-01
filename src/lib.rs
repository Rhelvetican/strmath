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
//! let s = Str::new("Hello, ");
//! let x = Str::from("World!");
//! let y = s + x;
//! ```
//!
//! ## Ops
//! The module that holds the operations. Refer to the [ops] module for more information.
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

pub mod fmt;
pub mod imp;
pub mod ops;

#[cfg(test)]
mod test;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Str(String);
