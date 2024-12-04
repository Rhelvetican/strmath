#[macro_export]
macro_rules! impl_from {
    ($($t:ty),+) => {
        $(
        impl From<$t> for $crate::Str {
            fn from(value: $t) -> $crate::Str {
                $crate::Str(::std::string::String::from(value))
            }
        }
        )+
    };
}
