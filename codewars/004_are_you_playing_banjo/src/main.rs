fn are_you_playing_banjo(name: &str) -> String {
    match name.chars().nth(0) {
        Some('r') | Some('R') => format!("{} plays banjo", name),
        _                     => format!("{} does not play banjo", name),
    }
}

fn main() {
    assert_eq!(are_you_playing_banjo("Martin"), "Martin does not play banjo");
    assert_eq!(are_you_playing_banjo("Rikke"), "Rikke plays banjo");
    assert_eq!(are_you_playing_banjo("bravo"), "bravo does not play banjo");
    assert_eq!(are_you_playing_banjo("rolf"), "rolf plays banjo");
}
