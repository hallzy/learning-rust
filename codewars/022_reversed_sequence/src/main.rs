fn reverse_seq(n: u32) -> Vec<u32> {
    if n == 0 {
        return Vec::new();
    }

    (1..=n).rev().collect()
}

fn main() {
    assert_eq!(reverse_seq(5), [5, 4, 3, 2, 1].to_vec());
}
