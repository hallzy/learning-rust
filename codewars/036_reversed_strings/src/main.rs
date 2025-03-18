fn solution(phrase: &str) -> String {
    phrase.chars().rev().collect()
}

fn main() {
    assert_eq!(solution("world"), "dlrow");
}
