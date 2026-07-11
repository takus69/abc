use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
        cs: [(usize, isize); n],
    }
    let mut ans: Vec<isize> = Vec::new();
    for i in 1..=m {
        let mut max = -1;
        for &(c, s) in &cs {
            if c == i {
                max = max.max(s);
            }
        }
        ans.push(max);
    }
    println!("{}", ans.iter().join(" "));
}