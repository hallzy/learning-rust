fn likes(names: &[&str]) -> String {
    match names {
        []           => format!("no one likes this"),
        [a]          => format!("{} likes this", a),
        [a,b]        => format!("{} and {} like this", a, b),
        [a,b,c]      => format!("{}, {} and {} like this", a, b, c),
        [a,b,r @ ..] => format!("{}, {} and {} others like this", a, b, r.len()),
    }
}

fn main() {
    assert_eq!(likes(&[]), "no one likes this");
    assert_eq!(likes(&["Peter"]), "Peter likes this");
    assert_eq!(likes(&["Jacob", "Alex"]), "Jacob and Alex like this");
    assert_eq!(
        likes(&["Max", "John", "Mark"]),
        "Max, John and Mark like this"
        );
    assert_eq!(
        likes(&["Alex", "Jacob", "Mark", "Max"]),
        "Alex, Jacob and 2 others like this"
        );
}
