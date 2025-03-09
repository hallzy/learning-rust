fn square_digits(num: u64) -> u64 {
    return num
        // Number to string
        .to_string()
        // String to iterator of chars
        .chars()
        .map(|c| {
            // Each char is converted to a digit
            let digit = c.to_digit(10).unwrap();
            // The digit is squared and converted to a string
            format!("{}", digit*digit)
        })
        // Convert iterating of Strings to a string
        .collect::<String>()
        // Parse string as an integer
        .parse()
        // Unwrap he Result
        .unwrap()
    ;
}

fn main() {
    assert_eq!(square_digits(9119), 811181, "\nFailed with num 9119");
}
