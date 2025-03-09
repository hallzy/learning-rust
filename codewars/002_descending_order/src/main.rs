fn descending_order(x: u64) -> u64 {
    let mut x_str1 : Vec<char> = x.to_string().chars().collect();
    x_str1.sort_by(|a, b| b.cmp(a));
    return String::from_iter(x_str1).parse::<u64>().unwrap();
}
