fn main() {
    for n in [10, 20, 30, 40, 50, 60] {
        let now = std::time::Instant::now();
        let val = fib_naive(n);
        let secs = now.elapsed().as_secs_f64();
        println!("{secs:20.8}s fib({n}) = {val}");
    }
}
