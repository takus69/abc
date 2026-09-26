use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        d: usize,
        x: [usize; n],
    }
    let mut ans: Vec<usize> = Vec::new();
    for i in 0..n {
        let mut flg = true;
        for j in 0..n {
            if i == j { continue; }
            if x[i].abs_diff(x[j]) < d {
                flg = false;
                break;
            }
        }
        if flg { ans.push(i+1); }
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}