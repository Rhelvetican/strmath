//! Substraction functions

use crate::mathstr::Str;
use std::ops::{Sub, SubAssign};

/// Substracts the `Str` by another `Str`.
impl Sub for Str {
    type Output = Str;
    fn sub(self, other: Self) -> Self::Output {
        Str(self.0.replacen(&other.0, "", 1))
    }
}

/// Substracts the `Str` by another `Str`.
impl SubAssign for Str {
    fn sub_assign(&mut self, other: Self) {
        self.0 = self.0.replacen(&other.0, "", 1)
    }
}

/// Substracts the `Str` by a `&str`.
impl Sub<&str> for Str {
    type Output = Str;
    fn sub(self, other: &str) -> Self::Output {
        Str(self.0.replacen(other, "", 1))
    }
}

/// Substracts the `Str` by a `&str`.
impl SubAssign<&str> for Str {
    fn sub_assign(&mut self, other: &str) {
        self.0 = self.0.replacen(other, "", 1)
    }
}

/// Substracts the `Str` by a `String`.
impl Sub<String> for Str {
    type Output = Str;
    fn sub(self, other: String) -> Self::Output {
        Str(self.0.replacen(&other, "", 1))
    }
}

/// Substracts the `Str` by a `String`.
impl SubAssign<String> for Str {
    fn sub_assign(&mut self, other: String) {
        self.0 = self.0.replacen(&other, "", 1)
    }
}

/// Substracts the `Str` by a `char`.
impl Sub<char> for Str {
    type Output = Str;
    fn sub(self, other: char) -> Self::Output {
        Str(self.0.replacen(other, "", 1))
    }
}

/// Substracts the `Str` by a `char`.
impl SubAssign<char> for Str {
    fn sub_assign(&mut self, other: char) {
        self.0 = self.0.replacen(other, "", 1)
    }
}
