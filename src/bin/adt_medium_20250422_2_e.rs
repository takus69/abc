use proconio::input_interactive;
use std::collections::HashSet;

fn main() {
    input_interactive! {
        n: usize,
    }
    let mut called: HashSet<usize> = HashSet::new();
    for _ in 0..=n {
        for i in 1..=(2*n+1) {
            if !called.contains(&i) {
                println!("{}", i);
                called.insert(i);
                break;
            }
        }
        input_interactive! {
            i: usize,
        }
        called.insert(i);
    }
}