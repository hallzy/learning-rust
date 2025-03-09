fn lovefunc(flower1: u16, flower2: u16) -> bool {
    let flower1_bool = flower1 % 2 == 0;
    let flower2_bool = flower2 % 2 == 0;

    flower1_bool != flower2_bool
}

fn main() {
    do_test(1, 4, true);
    do_test(2, 2, false);
    do_test(0, 1, true);
    do_test(0, 0, false);
}

fn do_test(f1: u16, f2: u16, exp: bool) {
    assert_eq!(lovefunc(f1, f2), exp, "\nFailed with flower1 {}, flower2 {}", f1, f2);
}
