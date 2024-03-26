#[cfg(test)]
pub mod test_module {
    use crate::Str;

    #[test]
    fn create_str() {
        let w = Str::new();
        assert_eq!(w.0, "".to_string());
    }

    // ... rest of the code ...
}
