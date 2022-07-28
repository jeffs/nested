type List = Vec<i32>;

pub fn append(mut items: List, mut suffix: List) -> List {
    items.append(&mut suffix);
    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append() {
        assert_eq!(vec![1, 2, 3, 4], append(vec![1, 2], vec![3, 4]));
    }

    #[test]
    fn test_append_repeatedly() {
        let items = [vec![8, 6], vec![7, 5], vec![3, 0, 9]]
            .into_iter()
            .fold(Vec::new(), append);
        assert_eq!(vec![8, 6, 7, 5, 3, 0, 9], items);
    }

    #[test]
    fn test_append_reuse() {
        let items = vec![1];
        assert_eq!(vec![1, 2], append(items, vec![2])); // OK

        //assert_eq!(vec![1, 2], append(items, vec![2])); // Won't compile.
    }
}
