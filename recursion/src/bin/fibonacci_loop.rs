fn fibonacci_loop(n: u64) -> u64 {
    let (mut a, mut b) = (0, 1);
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    a
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fibonacci_loop() {
        assert_eq!(fibonacci_loop(0), 0);
        assert_eq!(fibonacci_loop(1), 1);
        assert_eq!(fibonacci_loop(2), 1);
        assert_eq!(fibonacci_loop(3), 2);
        assert_eq!(fibonacci_loop(4), 3);
    }

    #[test]
    fn test_fibonacci_loop_big() {
        assert_eq!(fibonacci_loop(50), 12_586_269_025);
        assert_eq!(fibonacci_loop(60), 1_548_008_755_920);
    }
}

fn main() {
    for n in [10, 20, 30, 40, 50, 60] {
        let now = std::time::Instant::now();
        let value = fibonacci_loop(n);
        let secs = now.elapsed().as_secs_f64();
        println!("{secs:20.8}s fib({n}) = {value}");
    }
}
