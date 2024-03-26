use crate::Str;
use std::ops::{Mul, MulAssign};

impl Mul for Str {
    type Output = Str;
    fn mul(self, other: Self) -> Self::Output {
        Str(self.0.repeat(other.0.len()))
    }
}

impl Mul<isize> for Str {
    type Output = Str;
    fn mul(self, other: isize) -> Self::Output {
        if other < 0 {
            Str(self.0.chars().rev().collect::<String>().repeat(abs(other)))
        } else {
            Str(self.0.repeat(abs(other)))
        }
    }
}

impl MulAssign<isize> for Str {
    fn mul_assign(&mut self, other: isize) {
        if other < 0 {
            self.0 = self.0.chars().rev().collect::<String>().repeat(abs(other));
        } else {
            self.0 = self.0.repeat(abs(other));
        }
    }
}

impl Mul<Str> for isize {
    type Output = Str;
    fn mul(self, other: Str) -> Self::Output {
        if self < 0 {
            Str(other.0.chars().rev().collect::<String>().repeat(abs(self)))
        } else {
            Str(other.0.repeat(abs(self)))
        }
    }
}

impl MulAssign for Str {
    fn mul_assign(&mut self, other: Self) {
        self.0 = self.0.repeat(other.0.len());
    }
}

fn abs(a: isize) -> usize {
    let a = if a < 0 { -a } else { a };
    a as usize
}
