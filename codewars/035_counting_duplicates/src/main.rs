fn count_duplicates(text: &str) -> u32 {
    text
        .to_lowercase()
        .chars()
        .fold(std::collections::HashMap::new(), |mut acc, c| {
            let count = acc.entry(c).or_insert(0);
            *count += 1;
            acc
        })
        .iter()
        .fold(0, |acc, (_, &count)| {
            match count > 1 {
                true => acc + 1,
                false => acc,
            }
        })
}


fn main() {
    assert_eq!(count_duplicates("abcde"), 0);
    assert_eq!(count_duplicates("abcdea"), 1);
    assert_eq!(count_duplicates("indivisibility"), 1);
}
