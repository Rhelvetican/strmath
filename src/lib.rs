//! # StrMath `0.8.0`
//!
//! Performs mathematical operations with your strings!

mod macros;
mod utils;

#[doc(hidden)]
pub mod ops;
pub mod strmath;

//#[cfg(feature = "serde")]
#[doc(hidden)]
pub mod serde;

mod prelude {
    pub use crate::{ops::*, strmath::StrMath};
}

pub use prelude::*;
