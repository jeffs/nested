type List = Vec<i32>;

pub fn concatenate(mut items: List, mut suffix: List) -> List {
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

    #[test]
    fn test_concatenate_repeatedly() {
        let items = [vec![8, 6], vec![7, 5], vec![3, 0, 9]]
            .into_iter()
            .fold(Vec::new(), concatenate);
        assert_eq!(vec![8, 6, 7, 5, 3, 0, 9], items);
    }

    #[test]
    fn test_concatenate_reuse() {
        let items = vec![1];
        assert_eq!(vec![1, 2], concatenate(items, vec![2])); // OK
        //assert_eq!(vec![1, 2], concatenate(items, vec![2])); // Won't compile.
    }
}
