use std::ops::{Mul, MulAssign};

use crate::{strmath::ToMathStr, utils::Reverse, StrMath};

impl<'a> Mul<isize> for StrMath<'a> {
    type Output = StrMath<'a>;

    #[inline]
    fn mul(mut self, rhs: isize) -> Self::Output {
        if rhs.is_negative() {
            self.reverse();
        }

        let rhs = rhs.unsigned_abs();
        self * rhs
    }
}

impl<'a> Mul<usize> for StrMath<'a> {
    type Output = StrMath<'a>;

    #[inline]
    fn mul(self, rhs: usize) -> Self::Output {
        Self::new_owned(self.repeat(rhs))
    }
}

impl<'a> Mul for StrMath<'a> {
    type Output = StrMath<'a>;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self * rhs.to_math_str().len()
    }
}

impl MulAssign<isize> for StrMath<'_> {
    #[inline]
    fn mul_assign(&mut self, rhs: isize) {
        if rhs.is_negative() {
            self.reverse();
        }

        let rhs = rhs.unsigned_abs();
        *self *= rhs;
    }
}

impl MulAssign<usize> for StrMath<'_> {
    #[inline]
    fn mul_assign(&mut self, rhs: usize) {
        *(self.to_mut()) = self.inner.as_ref().repeat(rhs);
    }
}

impl<'a> MulAssign<StrMath<'a>> for StrMath<'a> {
    #[inline]
    fn mul_assign(&mut self, rhs: StrMath<'a>) {
        *self *= rhs.len();
    }
}
