use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
        d: [usize; n],
    }
    let mut set: HashSet<usize> = HashSet::new();
    for &ai in a.iter() {
        for &bi in b.iter() {
            set.insert(ai+bi);
        }
    }
    for &ci in c.iter() {
        for &di in d.iter() {
            if k < ci + di { continue; }
            let l = k - ci - di;
            if set.contains(&l) {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}