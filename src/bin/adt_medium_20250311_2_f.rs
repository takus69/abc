use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut h: usize,
        k: usize,
        s: Chars,
        xy: [(isize, isize); m],
    }
    let mut set: HashSet<(isize, isize)> = HashSet::new();
    for &(xi, yi) in xy.iter() {
        set.insert((xi, yi));
    } 
    let (mut x, mut y) = (0, 0);
    for (i, &si) in s.iter().enumerate() {
        match si {
            'R' => { x += 1; },
            'L' => { x -= 1; },
            'U' => { y += 1; },
            'D' => { y -= 1; },
            _ => {},
        }
        h -= 1;
        if h < k && set.contains(&(x, y)) {
            h = k;
            set.remove(&(x, y));
        }
        if h == 0 && i < n-1 {
            println!("No");
            std::process::exit(0);
        }
    }
    println!("Yes");
}