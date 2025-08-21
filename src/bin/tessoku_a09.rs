use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        abcd: [(usize, usize, usize,  usize); n],
    }
    let mut x: Vec<Vec<isize>> = vec![vec![0; w+1]; h+1];
    for &(a, b, c, d) in abcd.iter() {
        x[a-1][b-1] += 1;
        x[a-1][d] -= 1;
        x[c][b-1] -= 1;
        x[c][d] += 1;
    }
    let mut cumsum: Vec<Vec<isize>> = vec![vec![0; w+1]; h+1];
    for i in 0..h {
        for j in 0..w {
            cumsum[i+1][j+1] = cumsum[i+1][j] + x[i][j];
        }
    }
    for j in 0..w {
        for i in 0..h {
            cumsum[i+1][j+1] += cumsum[i][j+1];
        }
    }
    
    for i in 1..=h {
        println!("{}", cumsum[i][1..=w].iter().join(" "));
    }
}