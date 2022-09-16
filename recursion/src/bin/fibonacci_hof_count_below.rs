use std::iter::successors;

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

    #[test]
    fn test_fibonacci_hof_count_below() {
        assert_eq!(fibonacci_hof_count_below(0), 0);
        assert_eq!(fibonacci_hof_count_below(1), 1);
        assert_eq!(fibonacci_hof_count_below(2), 3);
        assert_eq!(fibonacci_hof_count_below(3), 4);
        assert_eq!(fibonacci_hof_count_below(4), 5);
    }
}

fn main() {
    for n in [10, 20, 30, 40, 50, 60] {
        let now = std::time::Instant::now();
        let value = fibonacci_hof(n);
        let secs = now.elapsed().as_secs_f64();
        println!("{secs:20.8}s fib({n}) = {value}");
    }

    println!();
    for n in [10, 20, 30, 40, 50, 60] {
        let now = std::time::Instant::now();
        let value = fibonacci_hof_count_below(n);
        let secs = now.elapsed().as_secs_f64();
        println!("{secs:20.8}s fib_count_below({n}) = {value}");
    }
}
