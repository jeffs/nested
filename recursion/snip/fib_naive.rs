fn fib_naive(n: u64) -> u64 {
    if n < 2 {
        n
    } else {
        fib_naive(n - 2) + fib_naive(n - 1)
    }
}
