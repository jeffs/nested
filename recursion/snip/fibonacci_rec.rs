fn fibonacci_rec(n: u64) -> u64 {
    fn imp(a: u64, b: u64, m: u64) -> u64 {
        match m {
            0 => a,
            _ => imp(b, a + b, m - 1),
        }
    }
    imp(0, 1, n)
}
