use crate::Str;
use std::ops::{Add, Rem, RemAssign};

impl Rem<char> for Str {
    type Output = Str;
    fn rem(self, other: char) -> Self::Output {
        Str(match self.0.rfind(other) {
            Some(i) => self.0[bump(i)..].to_string(),
            None => self.0.clone(),
        })
    }
}

impl RemAssign<char> for Str {
    fn rem_assign(&mut self, other: char) {
        match self.0.rfind(other) {
            Some(i) => self.0 = self.0[bump(i)..].to_string(),
            None => self.0 = self.0.clone(),
        }
    }
}

impl Rem<&str> for Str {
    type Output = Str;
    fn rem(self, other: &str) -> Self::Output {
        Str(match self.0.rfind(other) {
            Some(i) => self.0[bump(i)..].to_string(),
            None => self.0.clone(),
        })
    }
}

impl RemAssign<&str> for Str {
    fn rem_assign(&mut self, other: &str) {
        match self.0.rfind(other) {
            Some(i) => self.0 = self.0[bump(i)..].to_string(),
            None => self.0 = self.0.clone(),
        }
    }
}

impl Rem<String> for Str {
    type Output = Str;
    fn rem(self, other: String) -> Self::Output {
        Str(match self.0.rfind(other.as_str()) {
            Some(i) => self.0[bump(i)..].to_string(),
            None => self.0.clone(),
        })
    }
}

impl RemAssign<String> for Str {
    fn rem_assign(&mut self, other: String) {
        match self.0.rfind(other.as_str()) {
            Some(i) => self.0 = self.0[bump(i)..].to_string(),
            None => self.0 = self.0.clone(),
        }
    }
}

fn bump<T>(a: T) -> T
where
    T: Add<Output = T> + From<u8>,
{
    a + T::from(1)
}
