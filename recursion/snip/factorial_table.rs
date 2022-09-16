const FACTORIALS: [u32; 13] = [
    1, 1, 2, 6, 24, 120, 720, 5040, 40320,
    362880, 3628800, 39916800, 479001600,
];

fn factorial_table(n: u32) -> u32 {
    FACTORIALS[n as usize]
}
