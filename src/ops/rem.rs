use std::ops::{Rem, RemAssign};

use crate::StrMath;

impl<'a> Rem for StrMath<'a> {
    type Output = StrMath<'a>;

    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        Self::new_owned(
            self.split_once(rhs.as_ref())
                .map(|x| x.1.to_string())
                .unwrap_or(self.inner.into_owned()),
        )
    }
}

impl<'a> Rem<&'a str> for StrMath<'a> {
    type Output = StrMath<'a>;

    #[inline]
    fn rem(self, rhs: &'a str) -> Self::Output {
        Self::new_owned(
            self.split_once(rhs)
                .map(|x| x.1.to_string())
                .unwrap_or(self.inner.into_owned()),
        )
    }
}

impl<'a> Rem<char> for StrMath<'a> {
    type Output = StrMath<'a>;

    #[inline]
    fn rem(self, rhs: char) -> Self::Output {
        Self::new_owned(
            self.split_once(rhs)
                .map(|x| x.1.to_string())
                .unwrap_or(self.inner.into_owned()),
        )
    }
}

impl<'a> Rem<&[char]> for StrMath<'a> {
    type Output = StrMath<'a>;

    #[inline]
    fn rem(self, rhs: &[char]) -> Self::Output {
        Self::new_owned(
            self.split_once(rhs)
                .map(|x| x.1.to_string())
                .unwrap_or(self.inner.into_owned()),
        )
    }
}

impl RemAssign for StrMath<'_> {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        if let Some(x) = self.split_once(rhs.as_ref()).map(|x| x.1.to_string()) {
            *self = Self::new_owned(x)
        }
    }
}

impl<'a> RemAssign<&'a str> for StrMath<'a> {
    #[inline]
    fn rem_assign(&mut self, rhs: &'a str) {
        if let Some(x) = self.split_once(rhs).map(|x| x.1.to_string()) {
            *self = Self::new_owned(x)
        }
    }
}

impl RemAssign<char> for StrMath<'_> {
    #[inline]
    fn rem_assign(&mut self, rhs: char) {
        if let Some(x) = self.split_once(rhs).map(|x| x.1.to_string()) {
            *self = Self::new_owned(x)
        }
    }
}

impl<'a> RemAssign<&'a [char]> for StrMath<'a> {
    #[inline]
    fn rem_assign(&mut self, rhs: &'a [char]) {
        if let Some(x) = self.split_once(rhs).map(|x| x.1.to_string()) {
            *self = Self::new_owned(x)
        }
    }
}
