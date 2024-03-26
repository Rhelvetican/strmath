pub mod fmt;
pub mod imp;
pub mod ops;

#[cfg(test)]
mod test;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Str(String);
