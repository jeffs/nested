fn fibonaccis() -> impl Iterator<Item = u32> {
    successors(Some((0, 1)), |&(a, b)| Some((b, a + b)))
        .map(|(a, _)| a)
}

fn fibonacci_hof(n: u32) -> u32 {
    fibonaccis()
        .nth(n as usize)
        .expect("Fibonacci sequence should be inexhaustible")
}

fn fibonacci_hof_count_below(end: u32) -> u32 {
    fibonaccis().take_while(|&n| n < end).count() as u32
}
