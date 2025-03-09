fn disemvowel(s: &str) -> String {
    s.chars().filter(|c| !"aeiouAEIOU".contains(&c.to_string())).collect()
}

fn main() {
    assert_eq!(disemvowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!");
}
