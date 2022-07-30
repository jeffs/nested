Rust won’t compile ambiguous code that tries to mutate shared state.

    $ cargo test
       Compiling capture v0.1.0 (/home/jeff/git/nested/safety)
    error[E0506]: cannot assign to `suffix[_]` because it is borrowed
      --> src/lib.rs:55:9
       |
    53 |         let append34 = make_appender(&suffix);
       |                                      ------- borrow of `suffix[_]` occurs here
    54 |         assert_eq!(vec![1, 2, 3, 4], append34(vec![1, 2]));
    55 |         suffix[0] = 5; // Won't compile.
       |         ^^^^^^^^^^^^^ assignment to borrowed `suffix[_]` occurs here
    56 |         assert_eq!(vec![1, 2, 3, 4], append34(vec![1, 2]));
       |                                      -------- borrow later used here

    For more information about this error, try `rustc --explain E0506`.
