use std::ops::{Div, DivAssign};

use crate::StrMath;

impl<'a> Div for StrMath<'a> {
    type Output = StrMath<'a>;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self::new_owned(
            self.split_once(rhs.as_ref())
                .map(|x| x.0.to_string())
                .unwrap_or(self.inner.into_owned()),
        )
    }
}

impl<'a> Div<&'a str> for StrMath<'a> {
    type Output = StrMath<'a>;

    #[inline]
    fn div(self, rhs: &'a str) -> Self::Output {
        Self::new_owned(
            self.split_once(rhs)
                .map(|x| x.0.to_string())
                .unwrap_or(self.inner.into_owned()),
        )
    }
}

impl<'a> Div<char> for StrMath<'a> {
    type Output = StrMath<'a>;

    #[inline]
    fn div(self, rhs: char) -> Self::Output {
        Self::new_owned(
            self.split_once(rhs)
                .map(|x| x.0.to_string())
                .unwrap_or(self.inner.into_owned()),
        )
    }
}

impl<'a> Div<&[char]> for StrMath<'a> {
    type Output = StrMath<'a>;

    #[inline]
    fn div(self, rhs: &[char]) -> Self::Output {
        Self::new_owned(
            self.split_once(rhs)
                .map(|x| x.0.to_string())
                .unwrap_or(self.inner.into_owned()),
        )
    }
}

impl DivAssign for StrMath<'_> {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        if let Some(x) = self.split_once(rhs.as_ref()).map(|x| x.0.to_string()) {
            *self = Self::new_owned(x)
        }
    }
}

impl<'a> DivAssign<&'a str> for StrMath<'a> {
    #[inline]
    fn div_assign(&mut self, rhs: &'a str) {
        if let Some(x) = self.split_once(rhs).map(|x| x.0.to_string()) {
            *self = Self::new_owned(x)
        }
    }
}

impl DivAssign<char> for StrMath<'_> {
    #[inline]
    fn div_assign(&mut self, rhs: char) {
        if let Some(x) = self.split_once(rhs).map(|x| x.0.to_string()) {
            *self = Self::new_owned(x)
        }
    }
}

impl<'a> DivAssign<&'a [char]> for StrMath<'a> {
    #[inline]
    fn div_assign(&mut self, rhs: &'a [char]) {
        if let Some(x) = self.split_once(rhs).map(|x| x.0.to_string()) {
            *self = Self::new_owned(x)
        }
    }
}
