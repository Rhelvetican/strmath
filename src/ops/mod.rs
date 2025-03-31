use std::ops::Neg;

use crate::{declare_modules, utils::Reverse, StrMath};

declare_modules!(add, sub, mul, div, rem, deref, eq, ord, sh);

impl Neg for StrMath<'_> {
    type Output = Self;

    #[inline]
    fn neg(mut self) -> Self::Output {
        self.reverse();
        self
    }
}
