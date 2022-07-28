type List = Vec<i32>;

pub fn append(mut items: List, suffix: &[i32]) -> List {
    items.extend_from_slice(suffix);
    items
}

pub fn make_appender(suffix: &[i32]) -> impl Fn(List) -> List + '_ {
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
    #[rustfmt::skip]
    fn test_append_reuse() {
        let items = vec![1];
        assert_eq!(vec![1, 2], append(items, &[2])); // OK
    //  assert_eq!(vec![1, 2], append(items, &[2])); // Won't compile.
    }

    #[test]
    fn test_make_appender() {
        let append34 = make_appender(&[3, 4]);
        assert_eq!(vec![1, 2, 3, 4], append34(vec![1, 2]));
    }

    // Won't compile.
    #[test]
    #[rustfmt::skip]
    fn test_make_appender_dangle() {
    //  let append34 = {
    //      let suffix = vec![3, 4];
    //      make_appender(&suffix)
    //  };
    //  assert_eq!(vec![1, 2, 3, 4], append34(vec![1, 2]));
    }
}
