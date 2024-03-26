pub mod add;
pub mod div;
pub mod mul;
pub mod rem;
pub mod sub;

use crate::Str;
use std::ops::Neg;

impl Neg for Str {
    type Output = Str;
    fn neg(self) -> Self::Output {
        Str(self.0.chars().rev().collect())
    }
}
