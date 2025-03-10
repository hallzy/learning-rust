fn longest(a1: &str, a2: &str) -> String {
    let mut chars : Vec<char> = format!("{}{}", a1, a2).chars().collect();
    chars.sort();
    chars.dedup();
    chars.into_iter().collect()
}

fn testing(s1: &str, s2: &str, exp: &str) -> () {
    let ret = longest(s1, s2);

    println!("s1:{:?} s2:{:?}", s1, s2);
    println!("{:?} {:?}", ret, exp);
    println!("{}", ret == exp);
    assert_eq!(&ret, exp)
}

fn main() {
    testing("aretheyhere", "yestheyarehere", "aehrsty");
    testing("loopingisfunbutdangerous", "lessdangerousthancoding", "abcdefghilnoprstu");

}
