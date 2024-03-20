use std::{
    fmt::{Display, Formatter, Result},
    ops::*,
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Wrapper(String);

impl Wrapper {
    pub fn new() -> Self {
        Wrapper("".to_string())
    }
    pub fn from(s: &str) -> Self {
        Wrapper(s.to_string())
    }
}

impl Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Add for Wrapper {
    type Output = Wrapper;
    fn add(self, other: Self) -> Self::Output {
        Wrapper(self.0 + &other.0)
    }
}

impl AddAssign for Wrapper {
    fn add_assign(&mut self, other: Self) {
        self.0 += &other.0;
    }
}

impl Sub for Wrapper {
    type Output = Wrapper;
    fn sub(self, other: Self) -> Self::Output {
        let mut result = self.0;
        for c in other.0.chars() {
            result = result.replacen(c, "", 1);
        }
        Wrapper(result)
    }
}

impl SubAssign for Wrapper {
    fn sub_assign(&mut self, other: Self) {
        for c in other.0.chars() {
            self.0 = self.0.replacen(c, "", 1);
        }
    }
}

impl Mul for Wrapper {
    type Output = Wrapper;
    fn mul(self, other: Self) -> Self::Output {
        Wrapper(self.0.repeat(other.0.len()))
    }
}

impl MulAssign for Wrapper {
    fn mul_assign(&mut self, other: Self) {
        self.0 = self.0.repeat(other.0.len());
    }
}

impl<T: Into<isize> + PartialOrd + Copy> Mul<T> for Wrapper {
    type Output = Wrapper;
    fn mul(self, other: T) -> Self::Output {
        Wrapper(self.0.repeat(convert_to_usize(other.into())))
    }
}

impl<T: From<i32> + Into<isize> + PartialOrd + Copy> MulAssign<T> for Wrapper {
    fn mul_assign(&mut self, other: T) {
        if other < 0.into() {
            self.0 = self
                .0
                .chars()
                .rev()
                .collect::<String>()
                .repeat(convert_to_usize(other.into()));
        } else {
            self.0 = self.0.repeat(convert_to_usize(other.into()));
        }
    }
}

impl Neg for Wrapper {
    type Output = Wrapper;
    fn neg(self) -> Self::Output {
        Wrapper(self.0.chars().rev().collect())
    }
}

fn convert_to_usize(a: isize) -> usize {
    let a = if a < 0 { a * -1 } else { a };
    a as usize
}

#[cfg(test)]
mod test {
    #[test]
    fn create_wrapper() {
        let w = super::Wrapper::new();
        assert_eq!(w.0, "".to_string());
    }

    #[test]
    fn create_wrapper_from_string() {
        let w = super::Wrapper::from("hello");
        assert_eq!(w.0, "hello".to_string());
    }
}
