use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
    }
    let mut grid: Vec<Vec<usize>> = vec![vec![0; n]; n];
    let (mut r, mut c) = (0, (n-1)/2);
    let mut k = 1;
    grid[r][c] = k;
    for _ in 0..(n*n-1) {
        k += 1;
        if grid[(n+r-1)%n][(c+1)%n] == 0 {
            r = (n+r-1)%n;
            c = (c+1)%n;
            grid[r][c] = k;
        } else {
            r = (r+1)%n;
            grid[r][c] = k;
        }
    }
    for g in grid.iter() {
        println!("{}", g.iter().join(" "));
    }
}