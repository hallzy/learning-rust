// I found out after this that you can literally just do n*n*n to get the answer

fn row_sum_odd_numbers(n:i64) -> i64 {
    let first_index = (0..n).sum();
    let second_index = first_index + n;

    (first_index..second_index)
        .map(|x : i64 | 2*x + 1)
        .sum()
}

fn main() {
    assert_eq!(row_sum_odd_numbers(1), 1);
    assert_eq!(row_sum_odd_numbers(2), 8);
    assert_eq!(row_sum_odd_numbers(3), 27);
    assert_eq!(row_sum_odd_numbers(4), 64);
    assert_eq!(row_sum_odd_numbers(42), 74088);
}
