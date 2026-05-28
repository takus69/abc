use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    }

    fn check(x: usize, y: usize, set: &HashSet<(usize, usize)>) -> usize {
        let mut cnt = 0;
        for &(x2, y2) in set.iter() {
            if x==x2 || y==y2 { continue; }
            if set.contains(&(x, y2)) && set.contains(&(x2, y)) {
                cnt += 1;
            }
        }
        
        cnt
    }

    let set: HashSet<(usize, usize)> = xy.into_iter().collect();

    let mut ans = 0;
    for &(x, y) in &set {
        ans += check(x, y, &set);
    }

    println!("{}", ans/4);
}