use crate::encode::{decode, encode};

pub fn nth_fib_scalar(n: usize) -> u32 {
    let mut state: u32 = encode(0, 1);
    for _ in 0..n {
        let (first, second) = decode(state);
        state = encode(second, first + second);
    }
    decode(state).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_fib_scalar() {
        assert_eq!(nth_fib_scalar(6), 8);
    }
}
