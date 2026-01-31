use proconio::{input, marker::Chars};
use std::collections::VecDeque;
use itertools::Itertools;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            mut n: usize,
            mut c: usize,
            s: [Chars; n],
        }
        let mut bottom_wall: Vec<usize> = vec![0; n];
        for j in 0..n {
            for i in (0..n).rev() {
                if s[i][j] == '#' {
                    bottom_wall[j] = i;
                    break;
                }
            }
        }

        let mut dp: Vec<Vec<bool>> = vec![vec![false; n]; n];
        for i in 0..n {
            dp[i][c-1] = true;
        }

        for i in (0..(n-1)).rev() {
            for j in 0..n {
                if dp[i][j] { continue; }
                if s[i][j] == '#' && bottom_wall[j] == i {
                    let i2 = i+1;
                    for dj in [-1, 0, 1] {
                        let j2 = j as isize + dj;
                        if j2 < 0 || j2 >= n as isize { continue; }
                        let j2 = j2 as usize;
                        if dp[i2][j2] {
                            for i3 in 0..i2 {
                                dp[i3][j] = true;
                            }
                            break;
                        }
                    }
                }
                if s[i][j] == '.' {
                    let i2 = i+1;
                    for dj in [-1, 0, 1] {
                        let j2 = j as isize + dj;
                        if j2 < 0 || j2 >= n as isize { continue; }
                        let j2 = j2 as usize;
                        if dp[i2][j2] {
                            dp[i][j] = true;
                        }
                    }
                }
            }
        }

        let mut ans: Vec<usize> = Vec::new();
        for j in 0..n {
            if dp[0][j] {
                ans.push(1);
            } else { ans.push(0); }
        }

        println!("{}", ans.iter().join(""));
    }
}