use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let mut dp: Vec<Vec<usize>> = vec![vec![usize::MAX; w]; h];
    let mut que: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; w]; h];
    dp[0][0] = 1;
    que.push_front((0, 0));
    while let Some((i, j)) = que.pop_back() {
        if (i, j)==(h-1, w-1) { continue; }
        if visited[i][j] { continue; }
        visited[i][j] = true;
        if i+1 < h && c[i+1][j] == '.' {
            if dp[i+1][j] == usize::MAX {
                dp[i+1][j] = dp[i][j];
            } else {
                dp[i+1][j] += dp[i][j];
            }
            que.push_front((i+1, j));
        }
        if j+1 < w && c[i][j+1] == '.' {
            if dp[i][j+1] == usize::MAX {
                dp[i][j+1] = dp[i][j];
            } else {
                dp[i][j+1] += dp[i][j];
            }
            que.push_front((i, j+1));
        }
    }
    println!("{}", if dp[h-1][w-1] == usize::MAX { 0 } else { dp[h-1][w-1] });
}