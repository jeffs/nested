use std::collections::HashSet;

fn main() {
    let words: HashSet<_> = ["hello", "world"].into_iter().map(String::from).collect();
    let words: Vec<_> = words.into_iter().collect();
    println!("{}", words.join(" "));
}
