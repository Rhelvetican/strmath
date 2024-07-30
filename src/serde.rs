#[cfg(feature = "serde")]
use serde::Serialize;

use crate::MathString;

#[cfg(feature = "serde")]
impl Serialize for MathString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}
