#[cfg(test)]
pub mod test_module {
    use crate::Str;

    #[test]
    fn create_str() {
        let w = Str::new();
        assert_eq!(w.0, "");
    }

    #[test]
    fn create_str_from_string() {
        let w = Str::from("hello");
        assert_eq!(w.0, "hello");
    }

    #[test]
    fn add_str() {
        let w1 = Str::from("hello ");
        let w2 = Str::from("world");
        let w3 = w1 + w2;
        assert_eq!(w3.0, "hello world");
    }

    #[test]
    fn add_assign_str() {
        let mut w1 = Str::from("hello ");
        let w2 = Str::from("world");
        w1 += w2;
        assert_eq!(w1.0, "hello world");
    }

    #[test]
    fn add_i32() {
        let w1 = Str::from("hello ");
        let w2 = 3;
        let w3 = w1 + w2;
        assert_eq!(w3.0, "hello 3");
    }

    #[test]
    fn sub_str() {
        let w1 = Str::from("hello world");
        let w2 = Str::from(" world");
        let w3 = w1 - w2;
        assert_eq!(w3.0, "hello");
    }

    #[test]
    fn sub_assign_str() {
        let mut w1 = Str::from("hello world");
        let w2 = Str::from(" world");
        w1 -= w2;
        assert_eq!(w1.0, "hello");
    }

    #[test]
    fn mul_str() {
        let w1 = Str::from("hello ");
        let w2 = Str::from("world");
        let w3: Str = w1 * w2; // Add type annotation to specify the output type
        assert_eq!(w3.0, "hello hello hello hello hello ");
    }

    #[test]
    fn mul_str_with_int() {
        let w1 = Str::from("hello ");
        let w2: isize = 3;
        let w3: Str = w1 * Into::<isize>::into(w2);
        assert_eq!(w3.0, "hello hello hello ");
    }

    #[test]
    fn mul_str_with_neg_int() {
        let w1 = Str::from("hello ");
        let w2: isize = -5;
        let w3: Str = w1 * w2;
        assert_eq!(w3.0, " olleh olleh olleh olleh olleh");
    }

    #[test]
    fn mul_assign_str() {
        let mut w1 = Str::from("hello ");
        let w2 = Str::from("world");
        w1 *= w2;
        assert_eq!(w1.0, "hello hello hello hello hello ");
    }

    #[test]
    fn mul_assign_str_with_int() {
        let mut w1 = Str::from("hello ");
        let w2: isize = 3;
        w1 *= w2;
        assert_eq!(w1.0, "hello hello hello ");
    }

    #[test]
    fn mul_assign_str_with_neg_int() {
        let mut w1 = Str::from("hello ");
        let w2: isize = -5;
        w1 *= w2;
        assert_eq!(w1.0, " olleh olleh olleh olleh olleh");
    }

    #[test]
    fn div() {
        let w1 = Str::from("hello world");
        let w2 = Str::from(" ");
        let w3 = w1 / w2;
        assert_eq!(w3.0, "hello");
    }

    #[test]
    fn div_assign() {
        let mut w1 = Str::from("hello world");
        let w2 = Str::from(" ");
        w1 /= w2;
        assert_eq!(w1.0, "hello");
    }

    #[test]
    fn div_char() {
        let w1 = Str::from("hello world");
        let w2: char = ' ';
        let w3 = w1 / w2;
        assert_eq!(w3.0, "hello");
    }

    #[test]
    fn div_assign_char() {
        let mut w1 = Str::from("hello world");
        let w2: char = ' ';
        w1 /= w2;
        assert_eq!(w1.0, "hello");
    }

    #[test]
    fn rem_str() {
        let w1 = Str::from("hello world");
        let w2: char = ' ';
        let w3 = w1 % w2;
        assert_eq!(w3.0, "world");
    }

    #[test]
    fn rem_assign_str() {
        let mut w1 = Str::from("hello world");
        let w2: char = ' ';
        w1 %= w2;
        assert_eq!(w1.0, "world");
    }

    #[test]
    fn neg_str() {
        let w1 = Str::from("hello world");
        let w2 = -w1;
        assert_eq!(w2.0, "dlrow olleh");
    }

    #[test]
    fn div_str_with_str() {
        let w1 = Str::from("hello world");
        let w2 = " ";
        let w3 = w1 / w2;
        assert_eq!(w3.0, "hello");
    }

    #[test]
    fn div_assign_str_with_str() {
        let mut w1 = Str::from("hello world");
        let w2 = " ";
        w1 /= w2;
        assert_eq!(w1.0, "hello");
    }

    #[test]
    fn rem_str_with_str() {
        let w1 = Str::from("hello world");
        let w2 = " ";
        let w3 = w1 % w2;
        assert_eq!(w3.0, "world");
    }

    #[test]
    fn rem_assign_str_with_str() {
        let mut w1 = Str::from("hello world");
        let w2 = " ";
        w1 %= w2;
        assert_eq!(w1.0, "world");
    }
}
