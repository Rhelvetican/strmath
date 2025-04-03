//! # StrMath
//!
//! The actual implementation for [`StrMath`].

use std::{
    borrow::Cow,
    fmt::{Arguments, Debug, Display, Formatter, Result, Write},
};

#[derive(Clone, PartialEq, Eq, Default, Hash)]
#[repr(transparent)]
/// The main type that implements mathematical operations for strings.
pub struct StrMath<'a> {
    pub(crate) inner: Cow<'a, str>,
}

impl StrMath<'static> {
    #[inline]
    /// Creates a new [`StrMath`] with an empty string slice.
    pub const fn new() -> Self {
        Self::new_slice("")
    }
}

impl<'a> StrMath<'a> {
    #[inline]
    #[must_use]
    /// Constructs a new [`StrMath`] with a string slice.
    pub const fn new_slice(s: &'a str) -> Self {
        Self {
            inner: Cow::Borrowed(s),
        }
    }

    #[inline]
    #[must_use]
    /// Constructs a new [`StrMath`] with an owned string value.
    pub const fn new_owned(s: String) -> Self {
        Self {
            inner: Cow::Owned(s),
        }
    }

    #[inline]
    #[must_use = "If you don't need the new value, you can just call drop(value) instead of using this."]
    /// Convert this [`StrMath`] into another [`StrMath`] with owned value.
    ///
    /// Note: this is an associated function, which means that you have to call it as StrMath::into_owned(s) instead of s.into_owned().
    /// This is so that there is no conflict with a method on the inner type.    
    pub fn into_owned(inst: Self) -> Self {
        Self {
            inner: Cow::Owned(inst.inner.into_owned()),
        }
    }
}

impl AsRef<str> for StrMath<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.inner.as_ref()
    }
}

impl AsMut<String> for StrMath<'_> {
    #[inline]
    fn as_mut(&mut self) -> &mut String {
        self.inner.to_mut()
    }
}

impl<'a> From<&'a str> for StrMath<'a> {
    #[inline]
    fn from(value: &'a str) -> Self {
        Self::new_slice(value)
    }
}

impl Debug for StrMath<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "{}", self.as_ref())
    }
}

impl Display for StrMath<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "{}", self.as_ref())
    }
}

/// Convert types into [`StrMath`].
pub trait ToMathStr<'a> {
    /// Convert this data into [`StrMath`].
    fn to_math_str(self) -> StrMath<'a>;
}

macro_rules! impl_for_to_str_types {
    ($($ty:ty),+) => {
        $(
        impl<'a> $crate::strmath::ToMathStr<'a> for $ty {
            #[inline]
            fn to_math_str(self) -> StrMath<'a> {
                StrMath::new_owned(self.to_string())
            }
        }
        )+
    };
}

use std::{rc::Rc, sync::Arc};

impl_for_to_str_types!(
    i8,
    u8,
    i16,
    u16,
    i32,
    u32,
    i64,
    u64,
    isize,
    usize,
    i128,
    u128,
    f32,
    f64,
    Box<str>,
    Rc<str>,
    Arc<str>
);

impl<'a> ToMathStr<'a> for &'a str {
    #[inline]
    fn to_math_str(self) -> StrMath<'a> {
        StrMath::new_slice(self)
    }
}

impl<'a> ToMathStr<'a> for StrMath<'a> {
    #[inline]
    fn to_math_str(self) -> StrMath<'a> {
        self
    }
}

impl Write for StrMath<'_> {
    #[inline]
    fn write_fmt(&mut self, args: Arguments<'_>) -> Result {
        self.inner.to_mut().write_fmt(args)
    }

    #[inline]
    fn write_str(&mut self, s: &str) -> Result {
        self.inner.to_mut().write_str(s)
    }

    #[inline]
    fn write_char(&mut self, c: char) -> Result {
        self.inner.to_mut().write_char(c)
    }
}
