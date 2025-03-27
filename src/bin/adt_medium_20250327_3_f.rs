use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }
    let mut b: Vec<Vec<char>> = vec![vec!['.'; n]; n];
    for i in 0..(n/2) {
        for j in i..(n-i) {
            match (i+1)%4 {
                0 => {
                    b[i][j] = a[i][j];
                    b[n-i-1][j] = a[n-i-1][j];
                    b[j][i] = a[j][i];
                    b[j][n-i-1] = a[j][n-i-1];
                }
                1 => {
                    b[j][n-i-1] = a[i][j];
                    b[j][i] = a[n-i-1][j];
                    b[i][n-j-1] = a[j][i];
                    b[n-i-1][n-j-1] = a[j][n-i-1];
                },
                2 => {
                    b[n-i-1][n-j-1] = a[i][j];
                    b[i][n-j-1] = a[n-i-1][j];
                    b[n-j-1][n-i-1] = a[j][i];
                    b[n-j-1][i] = a[j][n-i-1];
                },
                3 => {
                    b[n-j-1][i] = a[i][j];
                    b[n-j-1][n-i-1] = a[n-i-1][j];
                    b[n-i-1][j] = a[j][i];
                    b[i][j] = a[j][n-i-1];
                },
                _ => {},
            }
        }
    }
    for bi in b.iter() {
        println!("{}", bi.iter().join(""));
    }
}