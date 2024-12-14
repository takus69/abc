use proconio::input;
use std::collections::{BinaryHeap, VecDeque};
use std::cmp::Reverse;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: u128,
        mut p: usize,
        mut q: usize,
        s: [[u128; w]; h],
    }
    p -= 1;
    q -= 1;
    let mut visited: Vec<Vec<bool>> = vec![vec![false; w]; h];
    let mut heap: BinaryHeap<(Reverse<u128>, usize, usize)> = BinaryHeap::new();  // 吸収した隣接した場所
    let mut que: VecDeque<(usize, usize)> = VecDeque::new();  // 吸収する場所
    que.push_front((p, q));
    visited[p][q] = true;
    let mut ans: u128 = 0;
    while !que.is_empty() {
        let (i, j) = que.pop_back().unwrap();
        ans += s[i][j];  // 吸収
        for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let i2 = i as isize + di;
            let j2 = j as isize + dj;
            if i2 < 0 || i2 >= h as isize || j2 < 0 || j2 >= w as isize { continue; }
            let i2 = i2 as usize;
            let j2 = j2 as usize;
            if visited[i2][j2] { continue; }
            heap.push((Reverse(s[i2][j2]), i2, j2));
            visited[i2][j2] = true;
        }
        // 吸収できる場所を確認
        if heap.is_empty() { break; }
        let (Reverse(y), i, j) = heap.pop().unwrap();
        if ans > y * x {
            que.push_front((i, j));
        } else {
            break;
        }
    }
    println!("{}", ans);
}