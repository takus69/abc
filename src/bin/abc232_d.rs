use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let mut que: VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; w]; h];
    que.push_back((0, 0, 1));
    visited[0][0] = true;
    let mut ans = 1;
    while let Some((i, j, d)) = que.pop_front() {
        for (di, dj) in [(1, 0), (0, 1)] {
            let i2 = i+di;
            let j2 = j+dj;
            if i2 >= h || j2 >= w { continue; }
            if c[i2][j2] == '#' || visited[i2][j2] { continue; }
            que.push_back((i2, j2, d+1));
            visited[i2][j2] = true;
            ans = ans.max(d+1);
        }
    }
    println!("{}", ans);
}
