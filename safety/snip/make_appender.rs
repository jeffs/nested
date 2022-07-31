// Rust make_appender

pub fn make_appender(suffix: &[i32]) -> impl Fn(Vec<i32>) -> Vec<i32> + '_ {
    |items| append(items, suffix)
}

fn test_make_appender() {
    let append34 = make_appender(&[3, 4]);
    assert_eq!(vec![1, 2, 3, 4], append34(vec![1, 2]));
}
