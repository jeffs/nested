fn fibonaccis() -> impl Iterator<Item = u64> {
    successors(Some((0, 1)), |&(a, b)| Some((b, a + b)))
        .map(|(a, _)| a)
}

fn fibonacci_hof(n: u64) -> u64 {
    fibonaccis()
        .nth(n as usize)
        .expect("Fibonacci sequence should be inexhaustible")
}

fn fibonacci_hof_count_below(end: u64) -> u64 {
    fibonaccis().take_while(|&n| n < end).count() as u64
}
