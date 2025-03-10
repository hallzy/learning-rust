fn is_triangle(a: i64, b: i64, c: i64) -> bool {
    let x = a + b > c;
    let y = a + c > b;
    let z = b + c > a;

    x && y && z
}

fn main() {
    assert_eq!(is_triangle(1, 2, 2), true);
    assert_eq!(is_triangle(7, 2, 2), false);
    assert_eq!(is_triangle(1, 2, 3), false);
    assert_eq!(is_triangle(1, 3, 2), false);
    assert_eq!(is_triangle(3, 1, 2), false);
    assert_eq!(is_triangle(5, 1, 2), false);
    assert_eq!(is_triangle(1, 2, 5), false);
    assert_eq!(is_triangle(2, 5, 1), false);
    assert_eq!(is_triangle(4, 2, 3), true);
    assert_eq!(is_triangle(5, 1, 5), true);
    assert_eq!(is_triangle(2, 2, 2), true);
    assert_eq!(is_triangle(-1, 2, 3), false);
    assert_eq!(is_triangle(1, -2, 3), false);
    assert_eq!(is_triangle(1, 2, -3), false);
    assert_eq!(is_triangle(0, 2, 3), false);
}
