fn get_sum(a: i64, b: i64) -> i64 {
    match a < b {
        true  => (a..=b).sum(),
        false => (b..=a).sum(),
    }
}

fn main() {
    assert_eq!(get_sum(0, 1), 1);
    assert_eq!(get_sum(1, 2), 3);
    assert_eq!(get_sum(5, -1), 14);
    assert_eq!(get_sum(505, 4), 127759);
}
