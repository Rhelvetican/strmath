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
