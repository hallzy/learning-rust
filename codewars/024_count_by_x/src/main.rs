fn count_by(x: u32, n: u32) -> Vec<u32> {
    (1..=n).map(|n| n * x).collect()
}

// use itertools::Itertools;

fn main() {
    assertion(vec![1,2,3,4,5,6,7,8,9,10], (1, 10));
    assertion(vec![2,4,6,8,10], (2, 5));
    assertion(vec![3,6,9,12,15,18,21], (3, 7));
    assertion(vec![50,100,150,200,250], (50, 5));
    assertion(vec![100,200,300,400,500,600], (100, 6));
}

fn assertion(expected : Vec<u32>, inputs : (u32, u32)) {
    let actual = count_by(inputs.0, inputs.1);
    assert!(expected == actual, "\nTest failed!\n expected: {:?}\n actual: {:?}\n x: {}\n n: {}\n", expected, actual, inputs.0, inputs.1);
}
