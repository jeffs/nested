fn count_factor(mut scalar: u32, factor: u32) -> u32 {
    let mut count = 0;
    while scalar % factor == 0 {
        scalar /= factor;
        count += 1;
    }
    count
}

pub fn decode(scalar: u32) -> (u32, u32) {
    (count_factor(scalar, 2), count_factor(scalar, 3))
}
