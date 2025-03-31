use std::ops::{Shl, ShlAssign, Shr, ShrAssign};

use crate::StrMath;

macro_rules! impl_shr {
    ($($ty:ty),+) => {
        $(
            impl<'a> Shr<$ty> for StrMath<'a> {
                type Output = StrMath<'a>;

                fn shr(mut self, rhs: $ty) -> Self::Output {
                    let refmut = self.inner.to_mut();

                    for _ in 0..rhs {
                        refmut.pop();

                        if refmut.is_empty() {
                            break;
                        }
                    }

                    self
                }
            }

            impl ShrAssign<$ty> for StrMath<'_> {
                fn shr_assign(&mut self, rhs: $ty) {
                    let refmut = self.inner.to_mut();

                    for _ in 0..rhs {
                        refmut.pop();

                        if refmut.is_empty() {
                            break;
                        }
                    }
                }
            }
        )+
    }
}

impl_shr!(u8, u16, u32, u64, u128, usize);

macro_rules! impl_shr_len {
    ($($ty:ty),+) => {
        $(
        impl<'a> Shr<$ty> for StrMath<'a> {
                type Output = StrMath<'a>;

                fn shr(mut self, rhs: $ty) -> Self::Output {
                    let refmut = self.inner.to_mut();

                    for _ in 0..rhs.len() {
                        refmut.pop();

                        if refmut.is_empty() {
                            break;
                        }
                    }

                    self
                }
            }

        impl ShrAssign<$ty> for StrMath<'_> {
            fn shr_assign(&mut self, rhs: $ty) {
                let refmut = self.inner.to_mut();

                for _ in 0..rhs.len() {
                    refmut.pop();

                    if refmut.is_empty() {
                        break;
                    }
                }
            }
        }
        )+
    };
}

impl_shr_len!(&str, &String, &StrMath<'_>);

macro_rules! impl_shl {
    ($($ty:ty),+) => {
        $(
            impl<'a> Shl<$ty> for StrMath<'a> {
                type Output = StrMath<'a>;

                fn shl(mut self, rhs: $ty) -> Self::Output {
                    let refmut = self.inner.to_mut();

                    for _ in 0..rhs {
                        refmut.push('\x00');
                    }

                    self
                }
            }

            impl ShlAssign<$ty> for StrMath<'_> {
                fn shl_assign(&mut self, rhs: $ty) {
                    let refmut = self.inner.to_mut();

                    for _ in 0..rhs {
                        refmut.push('\x00')
                    }
                }
            }
        )+
    }
}

impl_shl!(u8, u16, u32, u64, u128, usize);

macro_rules! impl_shl_len {
    ($($ty:ty),+) => {
        $(
            impl<'a> Shl<$ty> for StrMath<'a> {
                type Output = StrMath<'a>;

                fn shl(mut self, rhs: $ty) -> Self::Output {
                    let refmut = self.inner.to_mut();

                    for _ in 0..rhs.len() {
                        refmut.push('\x00');
                    }

                    self
                }
            }

            impl ShlAssign<$ty> for StrMath<'_> {
                fn shl_assign(&mut self, rhs: $ty) {
                    let refmut = self.inner.to_mut();

                    for _ in 0..rhs.len() {
                        refmut.push('\x00')
                    }
                }
            }
        )+
    }
}

impl_shl_len!(&str, &String, &StrMath<'_>);
