use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }
    let mut b = vec![vec![0; h]; w];
    for i in 0..h {
        for j in 0..w {
            b[j][i] = a[i][j];
        }
    }
    for bi in b.iter() {
        println!("{}", bi.iter().join(" "));
    }
}