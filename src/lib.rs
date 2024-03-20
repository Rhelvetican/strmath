use std::{
    convert::From,
    fmt::{Display, Formatter, Result},
    ops::*,
};

#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) struct Wrapper(String);

#[allow(dead_code)]
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
    pub fn push(&mut self, s: &str) {
        self.0 += s;
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
unsafe impl Send for Wrapper {
    // This is safe because Wrapper is not a reference type
}

unsafe impl Sync for Wrapper {
    // This is safe because Wrapper is not a reference type
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

impl Div<char> for Wrapper {
    type Output = Wrapper;
    fn div(self, other: char) -> Self::Output {
        Wrapper(match self.0.find(other) {
            Some(i) => self.0[..i].to_string(),
            None => self.0.clone(),
        })
    }
}

impl DivAssign<char> for Wrapper {
    fn div_assign(&mut self, other: char) {
        match self.0.find(other) {
            Some(i) => self.0 = self.0[..i].to_string(),
            None => self.0 = self.0.clone(),
        }
    }
}

impl Rem<char> for Wrapper {
    type Output = Wrapper;
    fn rem(self, other: char) -> Self::Output {
        Wrapper(match self.0.rfind(other) {
            Some(i) => self.0[bump(i)..].to_string(),
            None => self.0.clone(),
        })
    }
}

impl RemAssign<char> for Wrapper {
    fn rem_assign(&mut self, other: char) {
        match self.0.rfind(other) {
            Some(i) => self.0 = self.0[bump(i)..].to_string(),
            None => self.0 = self.0.clone(),
        }
    }
}

impl Neg for Wrapper {
    type Output = Wrapper;
    fn neg(self) -> Self::Output {
        Wrapper(self.0.chars().rev().collect())
    }
}

impl Div<&str> for Wrapper {
    type Output = Wrapper;
    fn div(self, other: &str) -> Self::Output {
        Wrapper(match self.0.find(other) {
            Some(i) => self.0[..i].to_string(),
            None => self.0.clone(),
        })
    }
}

impl DivAssign<&str> for Wrapper {
    fn div_assign(&mut self, other: &str) {
        match self.0.find(other) {
            Some(i) => self.0 = self.0[..i].to_string(),
            None => self.0 = self.0.clone(),
        }
    }
}

impl Rem<&str> for Wrapper {
    type Output = Wrapper;
    fn rem(self, other: &str) -> Self::Output {
        Wrapper(match self.0.rfind(other) {
            Some(i) => self.0[bump(i)..].to_string(),
            None => self.0.clone(),
        })
    }
}

impl RemAssign<&str> for Wrapper {
    fn rem_assign(&mut self, other: &str) {
        match self.0.rfind(other) {
            Some(i) => self.0 = self.0[bump(i)..].to_string(),
            None => self.0 = self.0.clone(),
        }
    }
}

// Hacky fixes for stuffs
fn convert_to_usize(a: isize) -> usize {
    let a = if a < 0 { -a } else { a };
    a as usize
}

fn bump<T>(a: T) -> T
where
    T: Add<Output = T> + From<u8>,
{
    a + T::from(1)
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

    #[test]
    fn mul_assign_wrapper() {
        let mut w1 = super::Wrapper::from("hello ");
        let w2 = super::Wrapper::from("world");
        w1 *= w2;
        assert_eq!(w1.0, "hello hello hello hello hello ".to_string());
    }

    #[test]
    fn mul_assign_wrapper_with_int() {
        let mut w1 = super::Wrapper::from("hello ");
        let w2: isize = 3;
        w1 *= w2;
        assert_eq!(w1.0, "hello hello hello ".to_string());
    }

    #[test]
    fn mul_assign_wrapper_with_neg_int() {
        let mut w1 = super::Wrapper::from("hello ");
        let w2: isize = -5;
        w1 *= w2;
        assert_eq!(w1.0, " olleh olleh olleh olleh olleh".to_string());
    }

    #[test]
    fn div_wrapper() {
        let w1 = super::Wrapper::from("hello world");
        let w2: char = ' ';
        let w3 = w1 / w2;
        assert_eq!(w3.0, "hello".to_string());
    }

    #[test]
    fn div_assign_wrapper() {
        let mut w1 = super::Wrapper::from("hello world");
        let w2: char = ' ';
        w1 /= w2;
        assert_eq!(w1.0, "hello".to_string());
    }

    #[test]
    fn rem_wrapper() {
        let w1 = super::Wrapper::from("hello world");
        let w2: char = ' ';
        let w3 = w1 % w2;
        assert_eq!(w3.0, "world".to_string());
    }

    #[test]
    fn rem_assign_wrapper() {
        let mut w1 = super::Wrapper::from("hello world");
        let w2: char = ' ';
        w1 %= w2;
        assert_eq!(w1.0, "world".to_string());
    }

    #[test]
    fn neg_wrapper() {
        let w1 = super::Wrapper::from("hello world");
        let w2 = -w1;
        assert_eq!(w2.0, "dlrow olleh".to_string());
    }

    #[test]
    fn div_wrapper_with_str() {
        let w1 = super::Wrapper::from("hello world");
        let w2 = " ";
        let w3 = w1 / w2;
        assert_eq!(w3.0, "hello".to_string());
    }

    #[test]
    fn div_assign_wrapper_with_str() {
        let mut w1 = super::Wrapper::from("hello world");
        let w2 = " ";
        w1 /= w2;
        assert_eq!(w1.0, "hello".to_string());
    }

    #[test]
    fn rem_wrapper_with_str() {
        let w1 = super::Wrapper::from("hello world");
        let w2 = " ";
        let w3 = w1 % w2;
        assert_eq!(w3.0, "world".to_string());
    }

    #[test]
    fn rem_assign_wrapper_with_str() {
        let mut w1 = super::Wrapper::from("hello world");
        let w2 = " ";
        w1 %= w2;
        assert_eq!(w1.0, "world".to_string());
    }
}
