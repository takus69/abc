use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    }
    let (mut si, mut sj) = (0, 0);
    let (mut gi, mut gj) = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'S' {
                (si, sj) = (i, j);
            }
            if a[i][j] == 'G' {
                (gi, gj) = (i, j);
            }
        }
    }

    let mut que: VecDeque<(usize, usize, usize, usize)> = VecDeque::new();
    let mut visited: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; w]; h]; 2];
    que.push_front((0, si, sj, 0));
    visited[0][si][sj] = true;
    while let Some((x, i, j, cnt)) = que.pop_back() {
        for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let i2 = i as isize + di;
            let j2 = j as isize + dj;
            if i2 < 0 || i2 >= h as isize || j2 < 0 || j2 >= w as isize { continue; }
            let i2 = i2 as usize;
            let j2 = j2 as usize;
            if a[i2][j2] == 'G' {
                println!("{}", cnt+1);
                return;
            }
            if a[i2][j2]=='#' { continue; }
            if (x==0 && a[i2][j2]=='x') || (x==1 && a[i2][j2]=='o') { continue; }
            let x2 = if a[i2][j2]=='?' { (x+1)%2 } else { x };
            if visited[x2][i2][j2] { continue; }
            que.push_front((x2, i2, j2, cnt+1));
            visited[x2][i2][j2] = true;
        }

    }
    println!("-1");
}