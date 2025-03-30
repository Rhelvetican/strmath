#[macro_export]
#[doc(hidden)]
macro_rules! declare_modules {
    ($($id:ident),+) => {
        $(
        #[doc(hidden)]
        pub mod $id;
        )+
    }
}
