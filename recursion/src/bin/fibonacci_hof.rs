use std::iter::successors;

fn fibonacci_hof(n: u64) -> u64 {
    successors(Some((0, 1)), |&(a, b)| Some((b, a + b)))
        .nth(n as usize)
        .expect("Fibonacci sequence should be inexhaustible")
        .0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fibonacci_hof() {
        assert_eq!(fibonacci_hof(0), 0);
        assert_eq!(fibonacci_hof(1), 1);
        assert_eq!(fibonacci_hof(2), 1);
        assert_eq!(fibonacci_hof(3), 2);
        assert_eq!(fibonacci_hof(4), 3);
    }

    #[test]
    fn test_fibonacci_hof_big() {
        assert_eq!(fibonacci_hof(50), 12_586_269_025);
        assert_eq!(fibonacci_hof(60), 1_548_008_755_920);
    }
}

fn main() {
    for n in [10, 20, 30, 40, 50, 60] {
        let now = std::time::Instant::now();
        let value = fibonacci_hof(n);
        let secs = now.elapsed().as_secs_f64();
        println!("{secs:20.8}s fib({n}) = {value}");
    }
}
