fn validate_pin(pin: &str) -> bool {
    pin.chars().all(|c| c.is_digit(10)) && (pin.len() == 4 || pin.len() == 6)
}

fn main() {
    invalid_length_tests();
    non_digit_chars_tests();
    valid_pin_tests();
}

fn invalid_length_tests() {
    assert_eq!(validate_pin("1"), false);
    assert_eq!(validate_pin("12"), false);
    assert_eq!(validate_pin("123"), false);
    assert_eq!(validate_pin("12345"), false);
    assert_eq!(validate_pin("1234567"), false);
    assert_eq!(validate_pin("-1234"), false);
    assert_eq!(validate_pin("1.234"), false);
    assert_eq!(validate_pin("-1.234"), false);
    assert_eq!(validate_pin("00000000"), false);
}

fn non_digit_chars_tests() {
    assert_eq!(validate_pin("a234"), false);
    assert_eq!(validate_pin(".234"), false);
}

fn valid_pin_tests() {
    assert_eq!(validate_pin("1234"), true);
    assert_eq!(validate_pin("0000"), true);
    assert_eq!(validate_pin("1111"), true);
    assert_eq!(validate_pin("123456"), true);
    assert_eq!(validate_pin("098765"), true);
    assert_eq!(validate_pin("000000"), true);
    assert_eq!(validate_pin("123456"), true);
    assert_eq!(validate_pin("090909"), true);
}
