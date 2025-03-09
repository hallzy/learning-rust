fn century(year: u32) -> u32 {
    ((year - 1) / 100) + 1
}

fn do_test(year: u32, expected: u32) {
    let actual = century(year);
    assert_eq!(actual, expected, "\n\nFor year = {}\n expected: {}\n actual: {}", year, expected, actual);
}

fn main() {
    do_test(1905, 20);
    do_test(1700, 17);
    do_test(89, 1);
    do_test(100, 1);
    do_test(101, 2);
}
