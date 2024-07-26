//! # Operations
//! Available operations for `Str` type are:
//! * Add (`+`)
//! * AddAssign (`+=`)
//! * Sub (`-`)
//! * SubAssign (`-=`)
//! * Mul (`*`)
//! * MulAssign (`*=`)
//! * Div (`/`)
//! * DivAssign (`/=`)
//! * Rem (`%`)
//! * RemAssign (`%=`)
//!
//! [add]: crate::ops::add
//! [sub]: crate::ops::sub
//! [mul]: crate::ops::mul
//! [div]: crate::ops::div
//! [rem]: crate::ops::rem
//!
//! ## Addition functions
//! For detailed information on addition functions, refer to the [_add_][add] module.
//! ### Add
//! The `Add` trait is used to add a string to another string or anything that implements the trait `Display`.
//! ```no_run
//! use strmath::Str;
//! let s = Str::from("Hello, ");
//! let x = Str::from("World!");
//! let y = s + x;
//! println!("{}", y.to_str());
//! ```
//!
//! ### AddAssign
//! The `AddAssign` trait is used to add a string to another string.
//! ```no_run
//! use strmath::Str;
//! let mut s = Str::from("Hello, ");
//! let x = Str::from("World!");
//! s += x;
//! println!("{}", s.to_str());
//! ```
//!
//! ## Subtraction functions
//! For detailed information on subtraction functions, refer to the [_sub_][sub] module.
//! ### Sub
//! The `Sub` trait is used to subtract a string from another string.
//! ```no_run
//! use strmath::Str;
//! let s = Str::from("Hello, World!");
//! let x = Str::from("World!");
//! let y = s - x;
//! println!("{}", y.to_str());
//! ```
//!
//! ### SubAssign
//! The `SubAssign` trait is used to subtract a string from another string.
//! ```no_run
//! use strmath::Str;
//! let mut s = Str::from("Hello, World!");
//! let x = Str::from("World!");
//! s -= x;
//! println!("{}", s.to_str());
//! ```
//!
//! ## Multiplication functions
//! For detailed information on multiplication functions, refer to the [_mul_][mul] module.
//! ### Mul
//! The `Mul` trait is used to multiply a string with another string/chars.
//! ```no_run
//! use strmath::Str;
//! let s = Str::from("Hi ");
//! let x = 7;
//! let y = s * x;
//! println!("{}", y.to_str());
//! ```
//! ### MulAssign
//! The `MulAssign` trait is used to multiply a string with another string/chars.
//! ```no_run
//! use strmath::Str;
//! let mut s = Str::from("Hi ");
//! let x = 7;
//! s *= x;
//! println!("{}", s.to_str());
//! ```
//!
//! ## Division functions
//! For detailed information on division functions, refer to the [_div_][div] module.
//! ### Div
//! The `Div` trait is used to split a string at the first matching point, then return the first part.
//! ```no_run
//! use strmath::Str;
//! let s = Str::from("Hello, World!");
//! let x = ",";
//! let y = s / x;
//! println!("{}", y.to_str());
//! ```
//!
//! ### DivAssign
//! The `DivAssign` trait is used to split a string at the first matching point, then return the first part.
//! ```no_run
//! use strmath::Str;
//! let mut s = Str::from("Hello, World!");
//! let x = ",";
//! s /= x;
//! println!("{}", s.to_str());
//! ```
//!
//! ## Remainder functions
//! For detailed information on remainder functions, refer to the [_rem_][rem] module.
//! ### Rem
//! The `Rem` trait is used to split a string at the first matching point, then return the second part.
//! ```no_run
//! use strmath::Str;
//! let s = Str::from("Hello, World!");
//! let x = ",";
//! let y = s % x;
//! println!("{}", y.to_str());
//! ```
//!
//! ### RemAssign
//! The `RemAssign` trait is used to split a string at the first matching point, then return the second part.
//! ```no_run
//! use strmath::Str;
//! let mut s = Str::from("Hello, World!");
//! let x = ",";
//! s %= x;
//! println!("{}", s.to_str());
//! ```

pub mod add;
pub mod div;
pub mod mul;
pub mod rem;
pub mod sub;

use crate::mathstr::Str;
use std::ops::{Index, IndexMut, Neg, Range};

impl Neg for Str {
    type Output = Str;
    fn neg(self) -> Self::Output {
        Str(self.0.chars().rev().collect())
    }
}

impl Index<Range<usize>> for Str {
    type Output = str;
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<Range<usize>> for Str {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}
