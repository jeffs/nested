fn fib_naive(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        fib_naive(n - 2) + fib_naive(n - 1)
    }
}
