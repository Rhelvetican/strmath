use crate::Str;
use std::{
    fmt::Display,
    ops::{Add, AddAssign},
};

impl<T: Display> Add<T> for Str {
    type Output = Str;
    fn add(self, other: T) -> Self::Output {
        Str(self.0 + &other.to_string())
    }
}

impl<T: Display> AddAssign<T> for Str {
    fn add_assign(&mut self, other: T) {
        self.0 += &other.to_string();
    }
}
