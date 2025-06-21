use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        d: [usize; n-1],
    }
    for i in 0..(n-1) {
        let mut ans: Vec<usize> = Vec::new();
        for j in (i+1)..n {
            let mut tmp = 0;
            for k in i..j {
                tmp += d[k];
            }
            ans.push(tmp)
        }
        println!("{}", ans.iter().join(" "));
    }
}