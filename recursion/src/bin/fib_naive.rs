fn fib_naive(n: u64) -> u64 {
    if n < 2 {
        n
    } else {
        fib_naive(n - 2) + fib_naive(n - 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fib_naive() {
        assert_eq!(fib_naive(0), 0);
        assert_eq!(fib_naive(1), 1);
        assert_eq!(fib_naive(2), 1);
        assert_eq!(fib_naive(3), 2);
        assert_eq!(fib_naive(4), 3);
    }

    #[ignore]
    #[test]
    fn test_fib_naive_big() {
        assert_eq!(fib_naive(50), 12_586_269_025);
        assert_eq!(fib_naive(60), 956_722_026_041);
    }
}

fn main() {
    for n in [10, 20, 30, 40, 50, 60] {
        let now = std::time::Instant::now();
        let val = fib_naive(n);
        let secs = now.elapsed().as_secs_f64();
        println!("{secs:20.8}s fib({n}) = {val}");
    }
}
