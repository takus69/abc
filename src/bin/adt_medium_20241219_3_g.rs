use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    let mut mass: Vec<Vec<bool>> = vec![vec![false; m]; n];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut que: Vec<(usize, usize)> = Vec::new();
    que.push((1, 1));
    visited.insert((1, 1));
    mass[1][1] = true;

    while !que.is_empty() {
        let (i, j) = que.pop().unwrap();
        // 下
        for i2 in (i+1)..n {
            if s[i2][j] == '.' {
                mass[i2][j] = true;
            } else {
                if !visited.contains(&(i2-1, j)) {
                    visited.insert((i2-1, j));
                    que.push((i2-1, j));
                }
                break;
            }
        }
        // 上
        for i2 in (0..i).rev() {
            if s[i2][j] == '.' {
                mass[i2][j] = true;
            } else {
                if !visited.contains(&(i2+1, j)) {
                    visited.insert((i2+1, j));
                    que.push((i2+1, j));
                }
                break;
            }
        }
        // 右
        for j2 in (j+1)..m {
            if s[i][j2] == '.' {
                mass[i][j2] = true;
            } else {
                if !visited.contains(&(i, j2-1)) {
                    visited.insert((i, j2-1));
                    que.push((i, j2-1));
                }
                break;
            }
        }
        // 左
        for j2 in (0..j).rev() {
            if s[i][j2] == '.' {
                mass[i][j2] = true;
            } else {
                if !visited.contains(&(i, j2+1)) {
                    visited.insert((i, j2+1));
                    que.push((i, j2+1));
                }
                break;
            }
        }
    }
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if mass[i][j] {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}