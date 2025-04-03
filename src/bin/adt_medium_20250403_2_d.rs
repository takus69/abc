use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }
    let mut a2: Vec<Vec<usize>> = Vec::new();
    for ai in a.iter() {
        a2.push(ai.iter().map(|&x| x.to_string().parse().unwrap()).collect());
    }
    let mut b = a.clone();
    for i in 0..n {
        if i == 0 {
            b[i][1] = a[i][0];
        } else {
            b[i-1][0] = a[i][0];
        }
        if i == n-1 {
            b[i][n-2] = a[i][n-1];
        } else {
            b[i+1][n-1] = a[i][n-1];
        }
    }
    for j in 0..n {
        if j == n-1 {
            b[1][j] = a[0][j];
        } else {
            b[0][j+1] = a[0][j];
        }
        if j == 0 {
            b[n-2][0] = a[n-1][0];
        } else {
            b[n-1][j-1] = a[n-1][j]; 
        }
    }
    for bi in b.iter() {
        println!("{}", bi.iter().join(""));
    }
}