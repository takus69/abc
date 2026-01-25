use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut cnt: Vec<usize> = vec![1; n+1];
    for &(a, b) in ab.iter() {
        cnt[a] += 1;
        cnt[b] += 1;
    }
    let mut ans: Vec<usize> = Vec::new();
    for i in 1..=n {
        let c = n-cnt[i];
        if c >= 3 {
            ans.push(c*(c-1)*(c-2)/6);
        } else {
            ans.push(0);
        }
    }
    println!("{}", ans.iter().join(" "));
}