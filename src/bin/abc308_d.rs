use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut que: VecDeque<(usize, usize)> = VecDeque::new();
    let mut ans = "Yes";
    let mut visited = vec![vec![false; w]; h];
    let check = ['s', 'n', 'u', 'k', 'e'];
    if s[0][0] != 's' { println!("No"); std::process::exit(0); }
    que.push_front((0, 0));
    let mut cnt = 1;
    loop {
        let mut next_que: VecDeque<(usize, usize)> = VecDeque::new();
        while !que.is_empty() {
            let (i, j) = que.pop_back().unwrap();
            for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let i2 = i as isize + di;
                let j2 = j as isize + dj;
                if i2 < 0 || i2 >= h as isize || j2 < 0 || j2 >= w as isize { continue; }
                let i2 = i2 as usize;
                let j2 = j2 as usize;
                if visited[i2][j2] { continue; }
                if s[i2][j2] == check[cnt%5] {
                    next_que.push_front((i2, j2));
                    visited[i2][j2] = true;
                }
            }
        }
        if next_que.contains(&(h-1, w-1)) { break; }
        if next_que.is_empty() {
            ans = "No";
            break;
        } else {
            que = next_que;
            cnt += 1;
        }
    }
    println!("{}", ans);
}