use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut ans: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    for &(a, b) in ab.iter() {
        ans[a].push(b);
        ans[b].push(a);
    }
    for a in 1..=n {
        ans[a].sort();
    }
    for v in ans.iter().skip(1) {
        println!("{} {}", v.len(), v.iter().join(" "));
    }
}