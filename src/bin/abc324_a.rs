use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut set = HashSet::new();
    for ai in a.iter() {
        set.insert(ai);
    }
    if set.len() == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}