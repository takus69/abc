use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        ab: [(usize, usize); k],
    }
    let mut ans: Vec<usize> = Vec::new();
    let mut cnt: Vec<usize> = vec![0; n];
    for &(a, b) in ab.iter() {
        cnt[a-1] += 1;
        if cnt[a-1] == m {
            ans.push(a);
        }
    }
    if !ans.is_empty() {
        println!("{}", ans.iter().join(" "));
    }
}