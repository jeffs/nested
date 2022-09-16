fn fibonacci_hof(n: u32) -> u32 {
    successors(Some((0, 1)), |&(a, b)| Some((b, a + b)))
        .nth(n as usize)
        .expect("Fibonacci sequence should be inexhaustible")
        .0
}
