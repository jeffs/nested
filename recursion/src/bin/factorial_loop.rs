fn factorial_loop(n: u64) -> u64 {
    let mut result = 1;
    for i in 2..=n {
        result *= i;
    }
    result
}

fn main() {
    assert_eq!(factorial_loop(0), 1);
    assert_eq!(factorial_loop(20), 2432902008176640000);
}
