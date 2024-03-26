pub mod fmt;
pub mod from;
pub mod ops;

use std::str::Chars;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Str(String);

#[allow(dead_code)]
impl Str {
    pub fn new() -> Self {
        Str("".to_string())
    }
    pub fn from(s: &str) -> Self {
        Str(s.to_string())
    }
    pub fn from_string(s: String) -> Self {
        Str(s)
    }
    pub fn to_str(&self) -> &str {
        &self.0
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Str(String::with_capacity(capacity))
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
    pub fn chars(&self) -> Chars<'_> {
        self.0.chars()
    }
}

#[cfg(test)]
mod test {
    use crate::Str;

    #[test]
    fn create_str() {
        let w = Str::new();
        assert_eq!(w.0, "".to_string());
    }

    #[test]
    fn create_str_from_string() {
        let w = Str::from("hello");
        assert_eq!(w.0, "hello".to_string());
    }

    #[test]
    fn add_str() {
        let w1 = Str::from("hello ");
        let w2 = Str::from("world");
        let w3 = w1 + w2;
        assert_eq!(w3.0, "hello world".to_string());
    }

    #[test]
    fn add_assign_str() {
        let mut w1 = Str::from("hello ");
        let w2 = Str::from("world");
        w1 += w2;
        assert_eq!(w1.0, "hello world".to_string());
    }

    #[test]
    fn sub_str() {
        let w1 = Str::from("hello world");
        let w2 = Str::from(" world");
        let w3 = w1 - w2;
        assert_eq!(w3.0, "hello".to_string());
    }

    #[test]
    fn sub_assign_str() {
        let mut w1 = Str::from("hello world");
        let w2 = Str::from(" world");
        w1 -= w2;
        assert_eq!(w1.0, "hello".to_string());
    }

    #[test]
    fn mul_str() {
        let w1 = Str::from("hello ");
        let w2 = Str::from("world");
        let w3: Str = w1 * w2; // Add type annotation to specify the output type
        assert_eq!(w3.0, "hello hello hello hello hello ".to_string());
    }

    #[test]
    fn mul_str_with_int() {
        let w1 = Str::from("hello ");
        let w2: isize = 3;
        let w3: Str = w1 * Into::<isize>::into(w2);
        assert_eq!(w3.0, "hello hello hello ".to_string());
    }

    #[test]
    fn mul_str_with_neg_int() {
        let w1 = Str::from("hello ");
        let w2: isize = -5;
        let w3: Str = w1 * w2;
        assert_eq!(w3.0, " olleh olleh olleh olleh olleh".to_string());
    }

    #[test]
    fn mul_assign_str() {
        let mut w1 = Str::from("hello ");
        let w2 = Str::from("world");
        w1 *= w2;
        assert_eq!(w1.0, "hello hello hello hello hello ".to_string());
    }

    #[test]
    fn mul_assign_str_with_int() {
        let mut w1 = Str::from("hello ");
        let w2: isize = 3;
        w1 *= w2;
        assert_eq!(w1.0, "hello hello hello ".to_string());
    }

    #[test]
    fn mul_assign_str_with_neg_int() {
        let mut w1 = Str::from("hello ");
        let w2: isize = -5;
        w1 *= w2;
        assert_eq!(w1.0, " olleh olleh olleh olleh olleh".to_string());
    }

    #[test]
    fn div() {
        let w1 = Str::from("hello world");
        let w2 = Str::from(" ");
        let w3 = w1 / w2;
        assert_eq!(w3.0, "hello".to_string());
    }

    #[test]
    fn div_assign() {
        let mut w1 = Str::from("hello world");
        let w2 = Str::from(" ");
        w1 /= w2;
        assert_eq!(w1.0, "hello".to_string());
    }

    #[test]
    fn div_char() {
        let w1 = Str::from("hello world");
        let w2: char = ' ';
        let w3 = w1 / w2;
        assert_eq!(w3.0, "hello".to_string());
    }

    #[test]
    fn div_assign_char() {
        let mut w1 = Str::from("hello world");
        let w2: char = ' ';
        w1 /= w2;
        assert_eq!(w1.0, "hello".to_string());
    }

    #[test]
    fn rem_str() {
        let w1 = Str::from("hello world");
        let w2: char = ' ';
        let w3 = w1 % w2;
        assert_eq!(w3.0, "world".to_string());
    }

    #[test]
    fn rem_assign_str() {
        let mut w1 = Str::from("hello world");
        let w2: char = ' ';
        w1 %= w2;
        assert_eq!(w1.0, "world".to_string());
    }

    #[test]
    fn neg_str() {
        let w1 = Str::from("hello world");
        let w2 = -w1;
        assert_eq!(w2.0, "dlrow olleh".to_string());
    }

    #[test]
    fn div_str_with_str() {
        let w1 = Str::from("hello world");
        let w2 = " ";
        let w3 = w1 / w2;
        assert_eq!(w3.0, "hello".to_string());
    }

    #[test]
    fn div_assign_str_with_str() {
        let mut w1 = Str::from("hello world");
        let w2 = " ";
        w1 /= w2;
        assert_eq!(w1.0, "hello".to_string());
    }

    #[test]
    fn rem_str_with_str() {
        let w1 = Str::from("hello world");
        let w2 = " ";
        let w3 = w1 % w2;
        assert_eq!(w3.0, "world".to_string());
    }

    #[test]
    fn rem_assign_str_with_str() {
        let mut w1 = Str::from("hello world");
        let w2 = " ";
        w1 %= w2;
        assert_eq!(w1.0, "world".to_string());
    }
}
