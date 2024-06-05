//! Multiplication functions

use crate::Str;
use std::ops::{Mul, MulAssign};

/// Multiplies the `Str` by another `Str`.
impl Mul for Str {
    type Output = Str;
    fn mul(self, other: Self) -> Self::Output {
        Str(self.0.repeat(other.0.len()))
    }
}

/// Multiplies the `Str` by an `isize`.
impl Mul<isize> for Str {
    type Output = Str;
    fn mul(self, other: isize) -> Self::Output {
        match other {
            0 => Str::new(),
            ..=0 => Str(self.0.chars().rev().collect::<String>().repeat(abs(other))),
            1.. => Str(self.0.repeat(abs(other))),
        }
    }
}

/// Multiplies the `Str` by an `isize`.
impl MulAssign<isize> for Str {
    fn mul_assign(&mut self, other: isize) {
        match other {
            0 => self.0 = String::new(),
            ..=0 => self.0 = self.0.chars().rev().collect::<String>().repeat(abs(other)),
            1.. => self.0 = self.0.repeat(abs(other)),
        };
    }
}

/// Multiplies an `isize` by a `Str`.
impl Mul<Str> for isize {
    type Output = Str;
    fn mul(self, other: Str) -> Self::Output {
        match self {
            0 => Str::new(),
            ..=0 => Str(other.0.chars().rev().collect::<String>().repeat(abs(self))),
            1.. => Str(other.0.repeat(abs(self))),
        }
    }
}

/// Multiplies an `Str` by a `Str`.
impl MulAssign for Str {
    fn mul_assign(&mut self, other: Self) {
        self.0 = self.0.repeat(other.0.len());
    }
}

/// Hacky absolute value function.
fn abs(a: isize) -> usize {
    let a = if a < 0 { -a } else { a };
    a as usize
}
