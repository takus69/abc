use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let mut que: VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; w]; h];
    let check = ['s', 'n', 'u', 'k', 'e'];
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    if s[0][0] == 's' {
        que.push_front((0, 0, 0));
        visited[0][0] = true;
    }

    while !que.is_empty() {
        let (i, j, c) = que.pop_back().unwrap();
        for &(di, dj) in dirs.iter() {
            let i2 = i as i32 + di;
            let j2 = j as i32 + dj;
            if i2 < 0 || i2 >= h as i32 || j2 < 0 || j2 >= w as i32 {
                continue;
            }
            let i2 = i2 as usize;
            let j2 = j2 as usize;
            if visited[i2][j2] { continue; }
            if check[(c+1)%5] == s[i2][j2] {
                que.push_front((i2, j2, c+1));
                visited[i2][j2] = true;
                if i2 == h-1 && j2 == w-1 {
                    println!("Yes");
                    std::process::exit(0);
                }
            }
        }
    }
    
    println!("No");
}