pub fn nth_fib(n: usize) -> u32 {
    let mut state = (0, 1);
    for _ in 0..n {
        state = (state.1, state.0 + state.1);
    }
    state.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_fib() {
        let got: Vec<u32> = (0..10).map(nth_fib).collect();
        assert_eq!(got, [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }
}
