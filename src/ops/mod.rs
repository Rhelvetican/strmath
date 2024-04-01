//! # Operations
//! Available operations for `Str` type are:
//! * Add (+)
//! * AddAssign (+=)
//! * Sub (-)
//! * SubAssign (-=)
//! * Mul (*)
//! * MulAssign (*=)
//! * Div (/)
//! * DivAssign (/=)
//! * Rem (%)
//! * RemAssign (%=)
//!
//! [add]: crate::ops::add
//! [sub]: crate::ops::sub
//! [mul]: crate::ops::mul
//! [div]: crate::ops::div
//! [rem]: crate::ops::rem
//!
//! ## Add
//! The `Add` trait is used to add a string to another string or anything that implements the trait `Display`.
//! ```rust
//! use strmath::Str;
//! let s = Str::from("Hello, ");
//! let x = Str::from("World!");
//!

#[cfg(feature = "add")]
pub mod add;
#[cfg(feature = "div")]
pub mod div;
#[cfg(feature = "mul")]
pub mod mul;
#[cfg(feature = "rem")]
pub mod rem;
#[cfg(feature = "sub")]
pub mod sub;

use crate::Str;
use std::ops::Neg;

impl Neg for Str {
    type Output = Str;
    fn neg(self) -> Self::Output {
        Str(self.0.chars().rev().collect())
    }
}
