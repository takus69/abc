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
    let mut items: HashSet<(isize, isize)> = HashSet::new();
    for &(x, y) in xy.iter() {
        items.insert((x, y));
    }
    let (mut x, mut y) = (0, 0);
    for &si in s.iter() {
        if h == 0 { println!("No"); return; }
        match si {
            'R' => { x += 1; },
            'L' => { x -= 1; },
            'U' => { y += 1; },
            'D' => { y -= 1; },
            _ => {},
        }
        h -= 1;
        if h < k && items.contains(&(x, y)) {
            h = k;
            items.remove(&(x, y));
        }
    }

    println!("Yes");
}