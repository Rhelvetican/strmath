use crate::StrMath;
use std::cmp::Ordering;

impl PartialOrd for StrMath<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StrMath<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        (***self).cmp(&***other)
    }
}
