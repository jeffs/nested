pub fn concatenate(mut items: Vec<i32>, mut suffix: Vec<i32>) -> Vec<i32> {
    items.append(&mut suffix);
    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concatenate() {
        assert_eq!(vec![1, 2, 3, 4], concatenate(vec![1, 2], vec![3, 4]));
    }
}
