pub use ::strmath::*;

#[test]
fn create_str() {
    let w = StrMath::new();
    assert_eq!(w, "");
}

#[test]
fn create_str_from_string() {
    let w = StrMath::from("hello");
    assert_eq!(w, "hello");
}

#[test]
fn add_str() {
    let w1 = StrMath::from("hello ");
    let w2 = StrMath::from("world");
    let w3 = w1 + w2;
    assert_eq!(w3, "hello world");
}

#[test]
fn add_assign_str() {
    let mut w1 = StrMath::from("hello ");
    let w2 = StrMath::from("world");
    w1 += w2;
    assert_eq!(w1, "hello world");
}

#[test]
fn add_i32() {
    let w1 = StrMath::from("hello ");
    let w2 = 3;
    let w3 = w1 + w2;
    assert_eq!(w3, "hello 3");
}

#[test]
fn sub_str() {
    let w1 = StrMath::from("hello world");
    let w2 = StrMath::from(" world");
    let w3 = w1 - w2;
    assert_eq!(w3, "hello");
}

#[test]
fn sub_assign_str() {
    let mut w1 = StrMath::from("hello world");
    let w2 = StrMath::from(" world");
    w1 -= w2;
    assert_eq!(w1, "hello");
}

#[test]
fn mul_str() {
    let w1 = StrMath::from("hello ");
    let w2 = StrMath::from("world");
    let w3 = w1 * w2;
    assert_eq!(w3, "hello hello hello hello hello ");
}

#[test]
fn mul_str_with_int() {
    let w1 = StrMath::from("hello ");
    let w2: isize = 3;
    let w3 = w1 * w2;
    assert_eq!(w3, "hello hello hello ");
}

#[test]
fn mul_str_with_neg_int() {
    let w1 = StrMath::from("hello ");
    let w2: isize = -5;
    let w3 = w1 * w2;
    assert_eq!(w3, " olleh olleh olleh olleh olleh");
}

#[test]
fn mul_assign_str() {
    let mut w1 = StrMath::from("hello ");
    let w2 = StrMath::from("world");
    w1 *= w2;
    assert_eq!(w1, "hello hello hello hello hello ");
}

#[test]
fn mul_assign_str_with_int() {
    let mut w1 = StrMath::from("hello ");
    let w2: isize = 3;
    w1 *= w2;
    assert_eq!(w1, "hello hello hello ");
}

#[test]
fn mul_assign_str_with_neg_int() {
    let mut w1 = StrMath::from("hello ");
    let w2: isize = -5;
    w1 *= w2;
    assert_eq!(w1, " olleh olleh olleh olleh olleh");
}

#[test]
fn div() {
    let w1 = StrMath::from("hello world");
    let w2 = StrMath::from(" ");
    let w3 = w1 / w2;
    assert_eq!(w3, "hello");
}

#[test]
fn div_assign() {
    let mut w1 = StrMath::from("hello world");
    let w2 = StrMath::from(" ");
    w1 /= w2;
    assert_eq!(w1, "hello");
}

#[test]
fn div_char() {
    let w1 = StrMath::from("hello world");
    let w2: char = ' ';
    let w3 = w1 / w2;
    assert_eq!(w3, "hello");
}

#[test]
fn div_assign_char() {
    let mut w1 = StrMath::from("hello world");
    let w2: char = ' ';
    w1 /= w2;
    assert_eq!(w1, "hello");
}

#[test]
fn rem_str() {
    let w1 = StrMath::from("hello world");
    let w2: char = ' ';
    let w3 = w1 % w2;
    assert_eq!(w3, "world");
}

#[test]
fn rem_assign_str() {
    let mut w1 = StrMath::from("hello world");
    let w2: char = ' ';
    w1 %= w2;
    assert_eq!(w1, "world");
}

#[test]
fn neg_str() {
    let w1 = StrMath::from("hello world");
    let w2 = -w1;
    assert_eq!(w2, "dlrow olleh");
}

#[test]
fn div_str_with_str() {
    let w1 = StrMath::from("hello world");
    let w2 = " ";
    let w3 = w1 / w2;
    assert_eq!(w3, "hello");
}

#[test]
fn div_assign_str_with_str() {
    let mut w1 = StrMath::from("hello world");
    let w2 = " ";
    w1 /= w2;
    assert_eq!(w1, "hello");
}

#[test]
fn rem_str_with_str() {
    let w1 = StrMath::from("hello world");
    let w2 = " ";
    let w3 = w1 % w2;
    assert_eq!(w3, "world");
}

#[test]
fn rem_assign_str_with_str() {
    let mut w1 = StrMath::from("hello world");
    let w2 = " ";
    w1 %= w2;
    assert_eq!(w1, "world");
}
