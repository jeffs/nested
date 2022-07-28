pub fn append(mut items: Vec<i32>, suffix: &[i32]) -> Vec<i32> {
    items.extend_from_slice(suffix);
    items
}

pub fn make_appender(suffix: &[i32]) -> impl Fn(Vec<i32>) -> Vec<i32> + '_ {
    move |prefix| append(prefix, suffix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append() {
        assert_eq!(vec![1, 2, 3, 4], append(vec![1, 2], &[3, 4]));
    }

    #[test]
    fn test_append_repeatedly() {
        let suffixes: [&[i32]; 3] = [&[8, 6], &[7, 5], &[3, 0, 9]];
        let items = suffixes.into_iter().fold(Vec::new(), append);
        assert_eq!(vec![8, 6, 7, 5, 3, 0, 9], items);
    }

    #[test]
    #[cfg(ignore)]
    fn test_append_reuse() {
        let items = vec![1];
        assert_eq!(vec![1, 2], append(items, &[2])); // OK
        assert_eq!(vec![1, 2], append(items, &[2])); // Won't compile.
    }

    #[test]
    fn test_make_appender() {
        let append34 = make_appender(&[3, 4]);
        assert_eq!(vec![1, 2, 3, 4], append34(vec![1, 2]));
    }

    #[test]
    #[cfg(ignore)]
    fn test_make_appender_dangle() {
        let append34 = {
            let suffix = vec![3, 4];
            make_appender(&suffix) // Won't compile.
        };
        assert_eq!(vec![1, 2, 3, 4], append34(vec![1, 2]));
    }

    #[test]
    #[cfg(ignore)]
    fn test_make_appender_mutate() {
        let mut suffix = [3, 4];
        let append34 = make_appender(&suffix);
        assert_eq!(vec![1, 2, 3, 4], append34(vec![1, 2]));
        suffix[0] = 5; // Won't compile.
        assert_eq!(vec![1, 2, 3, 4], append34(vec![1, 2]));
    }
}
