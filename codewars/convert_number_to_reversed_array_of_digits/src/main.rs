fn digitize(n: u64) -> Vec<u8> {
    return n
        // Convert to a String
        .to_string()
        // Creates an iterator over the characters
        .chars()
        // Reverses the iterator
        .rev()
        // Map each character to a u8 digit
        .map(|c| c.to_digit(10).unwrap() as u8)
        // Convert u8 iterator to a Vec
        .collect::<Vec<u8>>()
    ;
}

fn main() {
    assert_eq!(digitize(348597), vec![7,9,5,8,4,3]);
    assert_eq!(digitize(35231), vec![1,3,2,5,3]);
    assert_eq!(digitize(0), vec![0]);
}
