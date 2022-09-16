fn fibonacci_rec(n: u64) -> u64 {
    fn imp(a: u64, b: u64, m: u64) -> u64 {
        match m {
            0 => a,
            _ => imp(b, a + b, m - 1),
        }
    }
    imp(0, 1, n)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fibonacci_rec() {
        assert_eq!(fibonacci_rec(0), 0);
        assert_eq!(fibonacci_rec(1), 1);
        assert_eq!(fibonacci_rec(2), 1);
        assert_eq!(fibonacci_rec(3), 2);
        assert_eq!(fibonacci_rec(4), 3);
    }

    #[test]
    fn test_fibonacci_rec_big() {
        assert_eq!(fibonacci_rec(50), 12_586_269_025);
        assert_eq!(fibonacci_rec(60), 1_548_008_755_920);
    }
}

fn main() {
    for n in [10, 20, 30, 40, 50, 60] {
        let now = std::time::Instant::now();
        let value = fibonacci_rec(n);
        let secs = now.elapsed().as_secs_f64();
        println!("{secs:20.8}s fib({n}) = {value}");
    }
}
