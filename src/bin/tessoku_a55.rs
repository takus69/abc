use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        q: usize,
    }

    let mut set: BTreeSet<usize> = BTreeSet::new();
    for _ in 0..q {
        input! {
            c: usize,
            x: usize,
        }
        match c {
            1 => {
                set.insert(x);
            },
            2 => {
                set.remove(&x);
            },
            3 => {
                if let Some(y) = set.range(x..).next() {
                    println!("{}", y);
                } else {
                    println!("-1");
                }
            },
            _ => {},
        }
    }
}
