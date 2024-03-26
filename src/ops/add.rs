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
