fn square_sum(vec: Vec<i32>) -> i32 {
    vec.iter().fold(0, |acc, n| acc + n*n)
}

fn main() {
    assert_eq!(square_sum(vec![1, 2]), 5);
    assert_eq!(square_sum(vec![-1, -2]), 5);
    assert_eq!(square_sum(vec![5, 3, 4]), 50);
    assert_eq!(square_sum(vec![]), 0);
}
