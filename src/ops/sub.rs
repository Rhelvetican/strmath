use std::ops::{Sub, SubAssign};

use crate::{strmath::ToMathStr, StrMath};

impl<'a, T: ToMathStr<'a>> Sub<T> for StrMath<'a> {
    type Output = StrMath<'a>;

    fn sub(mut self, rhs: T) -> Self::Output {
        let inst = self.as_mut();
        *inst = inst.replacen(rhs.to_math_str().as_ref(), "", 1);
        self
    }
}

impl<'a, T: ToMathStr<'a>> SubAssign<T> for StrMath<'_> {
    fn sub_assign(&mut self, rhs: T) {
        let inst = self.as_mut();
        *inst = inst.replacen(rhs.to_math_str().as_ref(), "", 1);
    }
}
