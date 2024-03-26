use crate::Str;
use std::ops::{Add, AddAssign};

impl Add for Str {
    type Output = Str;
    fn add(self, other: Self) -> Self::Output {
        Str(self.0 + &other.0)
    }
}

impl AddAssign for Str {
    fn add_assign(&mut self, other: Self) {
        self.0 += &other.0;
    }
}

impl Add<&str> for Str {
    type Output = Str;
    fn add(self, other: &str) -> Self::Output {
        Str(self.0 + other)
    }
}

impl AddAssign<&str> for Str {
    fn add_assign(&mut self, other: &str) {
        self.0 += other;
    }
}

impl Add<String> for Str {
    type Output = Str;
    fn add(self, other: String) -> Self::Output {
        Str(self.0 + &other)
    }
}

impl AddAssign<String> for Str {
    fn add_assign(&mut self, other: String) {
        self.0 += &other;
    }
}
