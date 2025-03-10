fn make_upper_case(s: &str) -> String {
    s.to_uppercase()
}

fn main() {
    assert_eq!(make_upper_case("hello"), "HELLO");
}
