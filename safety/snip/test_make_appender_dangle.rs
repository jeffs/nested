// Rust test_make_appender_dangle

fn test_make_appender_dangle() {
    let append34 = {
        let suffix = vec![3, 4];
        make_appender(&suffix) // Won't compile.
    };
    assert_eq!(vec![1, 2, 3, 4], append34(vec![1, 2]));
}
