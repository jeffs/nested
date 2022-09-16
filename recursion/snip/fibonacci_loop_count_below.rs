fn fibonacci_loop_count_below(end: u32) -> u32 {
    let (mut a, mut b) = (0, 1);
    let mut n = 0;
    while a < end {
        n += 1;
        (a, b) = (b, a + b);
    }
    n
}
