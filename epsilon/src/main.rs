use std::collections::HashSet;

fn main() {
    let numbers = [f64::EPSILON / 3.0, f64::EPSILON / 2.0, 1.0];
    let unordered: HashSet<_> = numbers.into_iter().collect();
    println!("{}", numbers.iter().sum::<f64>());
    println!("{}", unordered.iter().sum::<f64>());
}
