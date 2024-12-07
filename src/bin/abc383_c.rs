use proconio::{input, marker::Chars};
use std::collections::{VecDeque, BinaryHeap};
use std::cmp::Reverse;

fn main() {
    input! {
        h: usize,
        w: usize,
        d: usize,
        s: [Chars; h],
    }
    let mut ans = 0;
    let mut heap: BinaryHeap<(Reverse<usize>, usize, usize)> = BinaryHeap::new();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'H' {
                heap.push((Reverse(0), i, j));
                visited[i][j] = true;
            }
        }
    }
    while !heap.is_empty() {
        let (Reverse(dist), i, j) = heap.pop().unwrap();
        if dist <= d {
            ans += 1;
        }
        for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let i2 = i as i32 + di;
            let j2 = j as i32 + dj;
            if i2 < 0 || i2 >= h as i32 || j2 < 0 || j2 >= w as i32 { continue; }
            let i2 = i2 as usize;
            let j2 = j2 as usize;
            if s[i2][j2] == 'H' { continue; }
            if !visited[i2][j2] && s[i2][j2] == '.' && dist+1 <= d {
                heap.push((Reverse(dist+1), i2, j2));
                visited[i2][j2] = true;
            }
        }
    }
    println!("{}", ans);
}