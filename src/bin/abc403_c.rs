use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
    }
    let mut all: Vec<bool> = vec![false; n+1];
    let mut set: HashSet<(usize, usize)> = HashSet::new();
    for _ in 0..q {
        input! {
            c: usize,
        }
        match c {
            1 => {
                input! {
                    x: usize,
                    y: usize,
                }
                set.insert((x, y));
            },
            2 => {
                input! {
                    x: usize,
                }
                all[x] = true;
            },
            3 => {
                input! {
                    x: usize,
                    y: usize,
                }
                if all[x] || set.contains(&(x, y)) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            },
            _ => { },
        }
    }
}