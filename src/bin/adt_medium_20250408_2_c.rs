use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
    }
    for i in 0..n {
        let mut ans: Vec<usize> = Vec::new();
        for j in 0..n {
            if a[i][j] == 1 {
                ans.push(j+1);
            }
        }
        println!("{}", ans.iter().join(" "));
    }
}