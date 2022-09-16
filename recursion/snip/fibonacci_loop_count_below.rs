fn fibonacci_loop_count_below(end: u64) -> u64 {
    let mut n = 0;
    let (mut a, mut b) = (0, 1);
    while a < end {
        n += 1;
        (a, b) = (b, a + b);
    }
    n
}
