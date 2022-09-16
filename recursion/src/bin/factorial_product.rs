fn factorial_product(n: u64) -> u64 {
    (2..=n).product()
}

fn main() {
    assert_eq!(factorial_product(0), 1);
    assert_eq!(factorial_product(20), 2432902008176640000);
}
