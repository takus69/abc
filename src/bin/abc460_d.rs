use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let mut ans: Vec<Vec<char>> = s.clone();

    // 1回目の操作 . => x
    for i in 0..h {
        for j in 0..w {
            if ans[i][j] == '#' {
                for &(i2, j2) in next(i, j, h, w).iter() {
                    if ans[i2][j2] == '.' {
                        ans[i2][j2] = 'x';
                    }
                }
            }
        }
    }
    // 1回目の操作 # => .
    for i in 0..h {
        for j in 0..w {
            if ans[i][j] == '#' {
                ans[i][j] = '.';
            }
        }
    }
    // 2回目の操作 . => #
    for i in 0..h {
        for j in 0..w {
            if ans[i][j] == 'x' {
                for &(i2, j2) in next(i, j, h, w).iter() {
                    if ans[i2][j2] == '.' {
                        ans[i2][j2] = '#';
                    }
                }
            }
        }
    }
    // 2回目の操作 # => .
    for i in 0..h {
        for j in 0..w {
            if ans[i][j] == 'x' {
                ans[i][j] = '.';
            }
        }
    }

    let mut que: VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if ans[i][j] == '#' {
                que.push_back((i, j, 0));
                visited[i][j] = true;
            }
        }
    }

    fn next(i: usize, j: usize, h: usize, w: usize) -> Vec<(usize, usize)> {
        let mut ret: Vec<(usize, usize)> = Vec::new();
        for (di, dj) in [(1, 1), (1, 0), (1, -1), (0, 1), (0, -1), (-1, 1), (-1, 0), (-1, -1)] {
            let i2 = i as isize + di;
            let j2 = j as isize + dj;
            if i2 < 0 || j2 < 0 || i2 >= h as isize || j2 >= w as isize { continue; }
            ret.push((i2 as usize, j2 as usize));
        }
        ret
    }

    while let Some((i, j, d)) = que.pop_front() {
        for &(i2, j2) in next(i, j, h, w).iter() {
            if visited[i2][j2] { continue; }
            que.push_back((i2, j2, d+1));
            visited[i2][j2] = true;
            if (d+1)%2==0 {
                ans[i2][j2] = '#';
            }
        }
    }
    for ai in &ans {
        println!("{}", ai.iter().collect::<String>());
    }
}