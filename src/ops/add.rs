use std::ops::{Add, AddAssign};

use crate::{strmath::ToMathStr, StrMath};

impl<'a, T: 'a + ToMathStr<'a>> Add<T> for StrMath<'a> {
    type Output = StrMath<'a>;

    #[inline]
    fn add(mut self, rhs: T) -> Self::Output {
        let s = rhs.to_math_str();
        self.inner.to_mut().push_str(&s);
        self
    }
}

impl<'a, T: 'a + ToMathStr<'a>> AddAssign<T> for StrMath<'a> {
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        self.inner.to_mut().push_str(&rhs.to_math_str());
    }
}
