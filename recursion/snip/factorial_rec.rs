fn factorial_rec(n: u64) -> u64 {
    if n < 2 {
        1
    } else {
        n * factorial_rec(n - 1)
    }
}
