use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        rc: [(usize, usize); m],
    }
    let mut set: HashSet<(usize, usize)> = HashSet::new();
    let mut ans = 0;
    for &(r, c) in rc.iter() {
        let mut flg = true;
        for (di, dj) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
            if set.contains(&(r+di, c+dj)) {
                flg = false;
            }
        }
        if flg {
            ans += 1;
            for (di, dj) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
                set.insert((r+di, c+dj));
            }
        }
    }
    println!("{}", ans);
}