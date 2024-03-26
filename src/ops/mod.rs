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
