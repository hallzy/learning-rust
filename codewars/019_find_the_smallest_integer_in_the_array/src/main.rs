fn find_smallest_int(arr: &[i32]) -> i32 {
    match arr.iter().min() {
        Some(&x) => x,
        _        => 0,
    }
}

fn main() {
    assert_eq!(find_smallest_int(&[34, 15, 88, 2]), 2);
    assert_eq!(find_smallest_int(&[34, -345, -1, 100]), -345);
}
