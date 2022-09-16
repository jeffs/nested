fn factorial_rec(n: u64) -> u64 {
    if n < 2 {
        1
    } else {
        n * factorial_rec(n - 1)
    }
}

fn main() {
    assert_eq!(factorial_rec(0), 1);
    assert_eq!(factorial_rec(20), 2432902008176640000);
}
