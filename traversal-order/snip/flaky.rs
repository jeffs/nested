fn test_flaky() {
    let set = HashSet::from(["Hello", "World"]);
    let vec: Vec<_> = set.into_iter().collect();
    assert_eq!("Hello World", vec.join(" ")); // Flaky!
}
