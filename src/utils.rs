use std::borrow::Cow;

use crate::StrMath;

/// Invert a string.
pub trait Reverse {
    /// Invert the given string in-place.
    fn reverse(&mut self);
}

impl Reverse for str {
    fn reverse(&mut self) {
        use core::str::from_utf8_unchecked;
        use unicode_segmentation::UnicodeSegmentation;

        // Algorithm are originally from `https://github.com/mbrubeck/unicode-reverse`.
        unsafe {
            let bytes = self.as_bytes_mut();
            let mut tail = &mut bytes[..];

            while let Some(l) = from_utf8_unchecked(tail)
                .graphemes(true)
                .next()
                .map(|s| s.len())
            {
                let (g, t) = tail.split_at_mut_unchecked(l);
                g.reverse();
                tail = t;
            }

            bytes.reverse();
        }
    }
}

impl Reverse for String {
    #[inline]
    fn reverse(&mut self) {
        self.as_mut_str().reverse();
    }
}

impl Reverse for Cow<'_, str> {
    #[inline]
    fn reverse(&mut self) {
        self.to_mut().reverse();
    }
}

impl Reverse for StrMath<'_> {
    #[inline]
    fn reverse(&mut self) {
        self.inner.reverse();
    }
}
