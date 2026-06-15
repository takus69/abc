use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            lr: [(usize, usize); m],
        }
        let mut max_d = 0;
        for &(l, r) in &lr {
            max_d = max_d.max(r-l+1);
        }
        let mut ans: Vec<usize> = Vec::new();
        for i in 0..n {
            ans.push(i%max_d+1);
        }
        println!("{}", ans.iter().join(" "));
    }
}