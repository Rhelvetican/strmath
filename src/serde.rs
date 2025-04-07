use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::fmt::{Formatter, Result};

use crate::StrMath;

impl<'a, 'de: 'a> Deserialize<'de> for StrMath<'a> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct StrVisitor;

        impl<'a, 'de: 'a> Visitor<'de> for StrVisitor {
            type Value = StrMath<'a>;

            fn expecting(&self, formatter: &mut Formatter) -> Result {
                formatter.write_str("Expecting a string.")
            }

            fn visit_str<E: Error>(self, v: &'a str) -> Result<Self::Value, E> {
                Ok(StrMath::new_slice(v))
            }
        }

        deserializer.deserialize_str(visitor)
    }
}
