fn fibonacci_rec(n: u32) -> u32 {
    fn imp(a: u32, b: u32, m: u32) -> u32 {
        match m {
            0 => a,
            _ => imp(b, a + b, m - 1),
        }
    }
    imp(0, 1, n)
}
