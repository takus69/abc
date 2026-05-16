use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        h: usize,
        w: usize,
    }
    let mut ans: Vec<Vec<usize>> = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            for (di, dj) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let i2 = i as i32 + di;
                let j2 = j as i32 + dj;
                if i2 >= 0 && i2 < h as i32 && j2 >= 0 && j2 < w as i32 {
                    ans[i][j] += 1;
                }
            }
        }
    }

    for a in &ans {
        println!("{}", a.iter().join(" "));
    }
}