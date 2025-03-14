fn tower_builder(n_floors: usize) -> Vec<String> {
    (1..=n_floors).map(|n| {
        let spaces = " ".repeat(n_floors - n);
        let stars = "*".repeat(2*(n - 1) + 1);
        format!("{}{}{}", spaces, stars, spaces)
    })
    .collect()
}


fn main() {
    assert_eq!(tower_builder(1), vec!["*"]);
    assert_eq!(tower_builder(2), vec![" * ", "***"]);
    assert_eq!(tower_builder(3), vec!["  *  ", " *** ", "*****"]);
}
