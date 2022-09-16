fn fibonacci_loop_count_below(end: u64) -> u64 {
    let mut n = 0;
    let (mut a, mut b) = (0, 1);
    while a < end {
        n += 1;
        (a, b) = (b, a + b);
    }
    n
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fibonacci_loop_count_below() {
        assert_eq!(fibonacci_loop_count_below(0), 0);
        assert_eq!(fibonacci_loop_count_below(1), 1);
        assert_eq!(fibonacci_loop_count_below(2), 3);
        assert_eq!(fibonacci_loop_count_below(3), 4);
        assert_eq!(fibonacci_loop_count_below(4), 5);
    }
}

fn main() {
    for n in [10, 20, 30, 40, 50, 60] {
        let now = std::time::Instant::now();
        let value = fibonacci_loop_count_below(n);
        let secs = now.elapsed().as_secs_f64();
        println!(
            "{secs:20.8}s fib_count_below({n}) = {value}"
        );
    }
}
