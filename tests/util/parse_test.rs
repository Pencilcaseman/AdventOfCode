use aoc::util::parse::*;

#[test]
fn test_parse_unsigned_immediate() {
    let test_input = "1234 1 2 3 NOPE 4 5";
    let mut bytes = test_input.bytes();

    assert_eq!(try_unsigned_immediate::<u32>(&mut bytes), Some(1234));
    assert_eq!(try_unsigned_immediate::<u32>(&mut bytes), Some(1));
    assert_eq!(try_unsigned_immediate::<u32>(&mut bytes), Some(2));
    assert_eq!(try_unsigned_immediate::<u32>(&mut bytes), Some(3));
    assert_eq!(try_unsigned_immediate::<u32>(&mut bytes), None);
}

#[test]
fn test_parse_unsigned() {
    let test_input = "abcd 1234 AAAA 5 10 15 20";
    let mut bytes = test_input.bytes();

    assert_eq!(try_unsigned::<u32>(&mut bytes), Some(1234));
    assert_eq!(try_unsigned::<u32>(&mut bytes), Some(5));
    assert_eq!(try_unsigned::<u32>(&mut bytes), Some(10));
    assert_eq!(try_unsigned::<u32>(&mut bytes), Some(15));
    assert_eq!(try_unsigned::<u32>(&mut bytes), Some(20));
    assert_eq!(try_unsigned::<u32>(&mut bytes), None);
}

#[test]
fn test_parse_signed_immediate() {
    let test_input = "-1234 1 -2 -3 NOPE 4 5";
    let mut bytes = test_input.bytes();

    assert_eq!(try_signed_immediate::<i32>(&mut bytes), Some(-1234));
    assert_eq!(try_signed_immediate::<i32>(&mut bytes), Some(1));
    assert_eq!(try_signed_immediate::<i32>(&mut bytes), Some(-2));
    assert_eq!(try_signed_immediate::<i32>(&mut bytes), Some(-3));
    assert_eq!(try_signed_immediate::<i32>(&mut bytes), None);
}

#[test]
fn test_parse_signed() {
    let test_input = "abcd -1234 AAAA -5 -10 -15 -20";
    let mut bytes = test_input.bytes();

    assert_eq!(try_signed::<i32>(&mut bytes), Some(-1234));
    assert_eq!(try_signed::<i32>(&mut bytes), Some(-5));
    assert_eq!(try_signed::<i32>(&mut bytes), Some(-10));
    assert_eq!(try_signed::<i32>(&mut bytes), Some(-15));
    assert_eq!(try_signed::<i32>(&mut bytes), Some(-20));
    assert_eq!(try_signed::<i32>(&mut bytes), None);
}
