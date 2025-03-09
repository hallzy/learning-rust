pub fn remove_char(s: &str) -> String {
    s[1..(s.len() - 1)].to_string()
}

fn main() {
    assert_eq!(remove_char("eloquent"), "loquen");
    assert_eq!(remove_char("country"), "ountr");
    assert_eq!(remove_char("person"), "erso");
    assert_eq!(remove_char("place"), "lac");
    assert_eq!(remove_char("ok"), "");
    assert_eq!(remove_char("ooopsss"), "oopss");
}
