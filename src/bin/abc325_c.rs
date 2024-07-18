use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let mut visited = vec![vec![false; w]; h];
    let mut que: VecDeque<(usize, usize)> = VecDeque::new();
    let mut ans = 0;
    let DIRS = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' && !visited[i][j] {
                ans += 1;
                que.push_front((i, j));
                visited[i][j] = true;
                while !que.is_empty() {
                    let (ii, jj) = que.pop_back().unwrap();
                    for (di, dj) in DIRS {
                        let i2 = (ii as i32) + di;
                        let j2 = (jj as i32) + dj;
                        if i2 < 0 || i2 >= h as i32 {
                            continue;
                        }
                        if j2 < 0 || j2 >= w as i32 {
                            continue;
                        }
                        let i2 = i2 as usize;
                        let j2 = j2 as usize;
                        if s[i2][j2] == '#' && !visited[i2][j2] {
                            que.push_front((i2, j2));
                            visited[i2][j2] = true;
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);
}