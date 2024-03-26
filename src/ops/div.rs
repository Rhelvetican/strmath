use crate::Str;
use std::ops::{Div, DivAssign};

impl Div<Str> for Str {
    type Output = Str;
    fn div(self, other: Str) -> Self::Output {
        Str(match self.0.find(&other.0) {
            Some(i) => self.0[..i].to_string(),
            None => self.0.clone(),
        })
    }
}

impl DivAssign<Str> for Str {
    fn div_assign(&mut self, other: Str) {
        match self.0.find(&other.0) {
            Some(i) => self.0 = self.0[..i].to_string(),
            None => self.0 = self.0.clone(),
        }
    }
}

impl Div<char> for Str {
    type Output = Str;
    fn div(self, other: char) -> Self::Output {
        Str(match self.0.find(other) {
            Some(i) => self.0[..i].to_string(),
            None => self.0.clone(),
        })
    }
}

impl DivAssign<char> for Str {
    fn div_assign(&mut self, other: char) {
        match self.0.find(other) {
            Some(i) => self.0 = self.0[..i].to_string(),
            None => self.0 = self.0.clone(),
        }
    }
}

impl Div<&str> for Str {
    type Output = Str;
    fn div(self, other: &str) -> Self::Output {
        Str(match self.0.find(other) {
            Some(i) => self.0[..i].to_string(),
            None => self.0.clone(),
        })
    }
}

impl DivAssign<&str> for Str {
    fn div_assign(&mut self, other: &str) {
        match self.0.find(other) {
            Some(i) => self.0 = self.0[..i].to_string(),
            None => self.0 = self.0.clone(),
        }
    }
}
