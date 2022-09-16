fn main() {
    for n in [10, 20, 30, 40, 50, 60] {
        let now = std::time::Instant::now();
        fib_naive(n);
        let secs = now.elapsed().as_secs_f64();
        println!("{n:-4} {secs}");
    }
}
