fn summation(n: i32) -> i32 {
    // For my own learning, apparently you can also just do this:
    // return  (1..=n).sum();
    // This is what I came up with though, because I did not know about the
    // sum() function. Good to know though!
    return  (1..=n).fold(0, |acc, n| acc + n);
}

fn main() {
    assert_eq!(summation(1), 1);
    assert_eq!(summation(8), 36);
    assert_eq!(summation(22), 253);
    assert_eq!(summation(100), 5050);
    assert_eq!(summation(213), 22791);
}
