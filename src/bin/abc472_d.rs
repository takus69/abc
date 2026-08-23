use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h],
    }
    let mut safe_h: Vec<usize> = Vec::new();
    for i in 0..h {
        let mut tmp = true;
        for j in 0..w {
            if s[i][j] == '#' {
                tmp = false;
                break;
            }
        }
        if tmp {
            safe_h.push(i);
        }
    }
    let mut safe_w: Vec<usize> = Vec::new();
    for j in 0..w {
        let mut tmp = true;
        for i in 0..h {
            if s[i][j] == '#' {
                tmp = false;
                break;
            }
        }
        if tmp {
            safe_w.push(j);
        }
    }

    let mut visited: Vec<Vec<bool>> = vec![vec![false; w]; h];
    let mut que: VecDeque<(usize, usize, usize)> = VecDeque::new();
    for &i in &safe_h {
        for &j in &safe_w {
            que.push_front((i, j, 0));
            visited[i][j] = true;
        }
    }
    let mut ans = 0;
    while let Some((i, j, d)) = que.pop_back() {
        if d <= k {
            ans += 1;
        }
        for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let i2 = i as isize + di;
            let j2 = j as isize + dj;
            if i2 < 0 || i2 >= h as isize || j2 < 0 || j2 >= w as isize { continue; }
            let i2 = i2 as usize;
            let j2 = j2 as usize;
            if visited[i2][j2] || s[i2][j2]=='#' { continue; }
            que.push_front((i2, j2, d+1));
            visited[i2][j2] = true;
        }
    }
    println!("{}", ans);
}