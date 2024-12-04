#[cfg(feature = "serde")]
use serde::{de::Visitor, Deserialize, Serialize};

#[cfg(feature = "serde")]
use crate::Str;

#[cfg(feature = "serde")]
impl Serialize for Str {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Str {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StrVisitor;
        impl Visitor<'_> for StrVisitor {
            type Value = Str;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "expected a string.")
            }
        }

        deserializer.deserialize_str(StrVisitor)
    }
}
