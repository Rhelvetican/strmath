use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
};

use crate::StrMath;

impl<'a> Deref for StrMath<'a> {
    type Target = Cow<'a, str>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for StrMath<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
