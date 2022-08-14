pub fn append(mut items: Vec<i32>, suffix: &[i32]) -> Vec<i32> {
    items.extend_from_slice(suffix);
    items
}

fn test_append() {
    assert_eq!(vec![1, 2, 3, 4], append(vec![1, 2], &[3, 4]));
}
