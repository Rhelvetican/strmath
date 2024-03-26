use crate::Str;
use std::ops::{Sub, SubAssign};

impl Sub for Str {
    type Output = Str;
    fn sub(self, other: Self) -> Self::Output {
        Str(self.0.replacen(&other.0, "", 1))
    }
}

impl SubAssign for Str {
    fn sub_assign(&mut self, other: Self) {
        self.0 = self.0.replacen(&other.0, "", 1)
    }
}
