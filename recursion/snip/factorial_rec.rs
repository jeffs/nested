fn factorial_rec(n: u32) -> u32 {
    if n < 2 {
        1
    } else {
        n * factorial_rec(n - 1)
    }
}
