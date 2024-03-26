use std::{
    convert::From,
    fmt::{Display, Formatter, Result},
    ops::*,
    str::Chars,
};

#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) struct Str(String);

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

impl From<&str> for Str {
    fn from(s: &str) -> Self {
        Str(s.to_string())
    }
}

impl From<String> for Str {
    fn from(s: String) -> Self {
        Str(s)
    }
}

impl Display for Str {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Add for Str {
    type Output = Str;
    fn add(self, other: Self) -> Self::Output {
        Str(self.0 + &other.0)
    }
}

impl AddAssign for Str {
    fn add_assign(&mut self, other: Self) {
        self.0 += &other.0;
    }
}

impl Sub for Str {
    type Output = Str;
    fn sub(self, other: Self) -> Self::Output {
        Str(self.0.replacen(&other.0, "", 1))
    }
}

impl SubAssign for Str {
    fn sub_assign(&mut self, other: Self) {
        self.0 = self.0.replacen(&other.0, "", 1)
    }
}

impl Mul for Str {
    type Output = Str;
    fn mul(self, other: Self) -> Self::Output {
        Str(self.0.repeat(other.0.len()))
    }
}

impl Mul<isize> for Str {
    type Output = Str;
    fn mul(self, other: isize) -> Self::Output {
        if other < 0 {
            Str(self
                .0
                .chars()
                .rev()
                .collect::<String>()
                .repeat(convert_to_usize(other)))
        } else {
            Str(self.0.repeat(convert_to_usize(other)))
        }
    }
}

impl MulAssign<isize> for Str {
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

impl Mul<Str> for isize {
    type Output = Str;
    fn mul(self, other: Str) -> Self::Output {
        if self < 0 {
            Str(other
                .0
                .chars()
                .rev()
                .collect::<String>()
                .repeat(convert_to_usize(self)))
        } else {
            Str(other.0.repeat(convert_to_usize(self)))
        }
    }
}

impl MulAssign for Str {
    fn mul_assign(&mut self, other: Self) {
        self.0 = self.0.repeat(other.0.len());
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

impl Neg for Str {
    type Output = Str;
    fn neg(self) -> Self::Output {
        Str(self.0.chars().rev().collect())
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
    fn div_str() {
        let w1 = Str::from("hello world");
        let w2: char = ' ';
        let w3 = w1 / w2;
        assert_eq!(w3.0, "hello".to_string());
    }

    #[test]
    fn div_assign_str() {
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
