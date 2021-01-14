$ cargo check
    Checking rust v0.1.0 (/Users/jeff/var/play/rust)
error[E0369]: cannot add `{integer}` to `&str`
 --> src/main.rs:2:25
  |
2 |     println!("{}", "42" + 1);
  |                    ---- ^ - {integer}
  |                    |
  |                    &str
