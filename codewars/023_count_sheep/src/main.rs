fn count_sheep(n: u32) -> String {
    (1..=n).fold(String::new(), |acc, n| format!("{}{} sheep...", acc, n))
}

fn main() {
    assert_eq!(count_sheep(0), "");
    assert_eq!(count_sheep(1), "1 sheep...");
    assert_eq!(count_sheep(2), "1 sheep...2 sheep...");
    assert_eq!(count_sheep(3), "1 sheep...2 sheep...3 sheep...");
}
