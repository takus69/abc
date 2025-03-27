use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut set: HashSet<usize> = HashSet::new();
    for &ai in a.iter() {
        set.insert(ai);
    }
    println!("{}", set.len());
}