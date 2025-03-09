fn get_count(string: &str) -> usize {
    let mut vowels_count: usize = 0;

    for c in string.chars() {
        match c {
            'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U' => vowels_count += 1,
            _   => (),
        }
    }

    vowels_count
}

fn main() {
    assert_eq!(get_count("abracadabra"), 5);
    assert_eq!(get_count("Steven Hall"), 3);
}
