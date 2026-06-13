use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
    }
    let mut ans: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    for i in 0..n {
        input! {
            k: usize,
            a: [usize; k],
        }
        for &ai in &a {
            ans[ai].push(i+1);
        }
    }
    for a in ans.iter().skip(1) {
        println!("{} {}", a.len(), a.iter().join(" "));
    }
}