use crate::StrMath;

impl PartialEq<str> for StrMath<'_> {
    fn eq(&self, other: &str) -> bool {
        **self == other
    }
}

impl PartialEq<&str> for StrMath<'_> {
    fn eq(&self, other: &&str) -> bool {
        **self == *other
    }
}
