use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    }
    let mut s: HashSet<(usize, usize)> = HashSet::new();
    for &(a, b, c, d) in abcd.iter() {
        for x in a..b {
            for y in c..d {
                s.insert((x, y));
            }
        }
    }
    println!("{}", s.len());
}