use std::{
    convert::From,
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
    pub fn from_string(s: String) -> Self {
        Wrapper(s)
    }
    pub fn to_str(&self) -> &str {
        &self.0
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Wrapper(String::with_capacity(capacity))
    }
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }
    pub fn reserve_exact(&mut self, additional: usize) {
        self.0.reserve_exact(additional);
    }
    pub fn reverse(&mut self) {
        self.0 = self.0.chars().rev().collect();
    }
}

impl From<&str> for Wrapper {
    fn from(s: &str) -> Self {
        Wrapper(s.to_string())
    }
}

impl From<String> for Wrapper {
    fn from(s: String) -> Self {
        Wrapper(s)
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
        Wrapper(self.0.replacen(&other.0, "", 1))
    }
}

impl SubAssign for Wrapper {
    fn sub_assign(&mut self, other: Self) {
        self.0 = self.0.replacen(&other.0, "", 1)
    }
}

impl Mul for Wrapper {
    type Output = Wrapper;
    fn mul(self, other: Self) -> Self::Output {
        Wrapper(self.0.repeat(other.0.len()))
    }
}

impl Mul<isize> for Wrapper {
    type Output = Wrapper;
    fn mul(self, other: isize) -> Self::Output {
        if other < 0 {
            Wrapper(
                self.0
                    .chars()
                    .rev()
                    .collect::<String>()
                    .repeat(convert_to_usize(other)),
            )
        } else {
            Wrapper(self.0.repeat(convert_to_usize(other)))
        }
    }
}

impl MulAssign<isize> for Wrapper {
    fn mul_assign(&mut self, other: isize) {
        if other < 0 {
            self.0 = self
                .0
                .chars()
                .rev()
                .collect::<String>()
                .repeat(convert_to_usize(other));
        } else {
            self.0 = self.0.repeat(convert_to_usize(other));
        }
    }
}

impl Mul<Wrapper> for isize {
    type Output = Wrapper;
    fn mul(self, other: Wrapper) -> Self::Output {
        if self < 0 {
            Wrapper(
                other
                    .0
                    .chars()
                    .rev()
                    .collect::<String>()
                    .repeat(convert_to_usize(self)),
            )
        } else {
            Wrapper(other.0.repeat(convert_to_usize(self)))
        }
    }
}

impl MulAssign for Wrapper {
    fn mul_assign(&mut self, other: Self) {
        self.0 = self.0.repeat(other.0.len());
    }
}

impl Neg for Wrapper {
    type Output = Wrapper;
    fn neg(self) -> Self::Output {
        Wrapper(self.0.chars().rev().collect())
    }
}

// Hacky fixes for stuffs
fn convert_to_usize(a: isize) -> usize {
    let a = if a < 0 { -a } else { a };
    a as usize
}

#[cfg(test)]
mod test {
    use crate::Wrapper;

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

    #[test]
    fn add_wrapper() {
        let w1 = super::Wrapper::from("hello ");
        let w2 = super::Wrapper::from("world");
        let w3 = w1 + w2;
        assert_eq!(w3.0, "hello world".to_string());
    }

    #[test]
    fn add_assign_wrapper() {
        let mut w1 = super::Wrapper::from("hello ");
        let w2 = super::Wrapper::from("world");
        w1 += w2;
        assert_eq!(w1.0, "hello world".to_string());
    }

    #[test]
    fn sub_wrapper() {
        let w1 = super::Wrapper::from("hello world");
        let w2 = super::Wrapper::from(" world");
        let w3 = w1 - w2;
        assert_eq!(w3.0, "hello".to_string());
    }

    #[test]
    fn sub_assign_wrapper() {
        let mut w1 = super::Wrapper::from("hello world");
        let w2 = super::Wrapper::from(" world");
        w1 -= w2;
        assert_eq!(w1.0, "hello".to_string());
    }

    #[test]
    fn mul_wrapper() {
        let w1 = super::Wrapper::from("hello ");
        let w2 = super::Wrapper::from("world");
        let w3: Wrapper = w1 * w2; // Add type annotation to specify the output type
        assert_eq!(w3.0, "hello hello hello hello hello ".to_string());
    }

    #[test]
    fn mul_wrapper_with_int() {
        let w1 = super::Wrapper::from("hello ");
        let w2: isize = 3;
        let w3: Wrapper = w1 * Into::<isize>::into(w2);
        assert_eq!(w3.0, "hello hello hello ".to_string());
    }

    #[test]
    fn mul_wrapper_with_neg_int() {
        let w1 = super::Wrapper::from("hello ");
        let w2: isize = -5;
        let w3: Wrapper = w1 * w2;
        assert_eq!(w3.0, " olleh olleh olleh olleh olleh".to_string());
    }
}
