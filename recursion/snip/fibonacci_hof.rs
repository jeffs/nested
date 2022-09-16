use std::iter::successors;

fn fibonacci_hof(n: u64) -> u64 {
    successors(Some((0, 1)), |&(a, b)| Some((b, a + b)))
        .nth(n as usize)
        .expect("Fibonacci sequence should be inexhaustible")
        .0
}
