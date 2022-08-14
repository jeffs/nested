fn test_make_appender_mutate() {
    let mut suffix = [3, 4];
    let append34 = make_appender(&suffix);
    assert_eq!(vec![1, 2, 3, 4], append34(vec![1, 2]));
    suffix[0] = 5; // Won't compile.
    assert_eq!(vec![1, 2, 3, 4], append34(vec![1, 2]));
}
