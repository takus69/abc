use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h],
    }

    fn bfs(si: usize, sj: usize, h: usize, w: usize, k: usize, s: &Vec<Vec<char>>) -> usize {
        // println!("(si, sj)=({}, {})", si, sj);
        let mut que: VecDeque<(usize, usize, usize, Vec<Vec<bool>>)> = VecDeque::new();
        let mut ans = 0;
        let mut visited = vec![vec![false; w]; h];
        visited[si][sj] = true;
        que.push_front((si, sj, 0, visited));
        while !que.is_empty() {
            // println!("que: {:?}", que);
            let (i, j, cnt, visited) = que.pop_back().unwrap();
            for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let i = i as i64 + di;
                let j = j as i64 + dj;
                if i < 0 || i >= h as i64 || j < 0 || j >= w as i64 { continue; }
                let i = i as usize;
                let j = j as usize;
                if s[i][j] == '#' || visited[i][j] || cnt >= k { continue; }
                // println!("(i, j)=({}, {}), cnt: {}", i, j, cnt+1);
                if cnt+1 == k {
                    // println!("add: (i, j)=({}, {}), cnt: {}", i, j, cnt+1);
                    ans += 1;
                    continue;
                }
                let mut visited2 = visited.clone();
                visited2[i][j] = true;
                que.push_front((i, j, cnt+1, visited2));
            }
        }
        // println!("ans: {}", ans);
        ans
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' { continue; }
            ans += bfs(i, j, h, w, k, &s);
        }
    }
    println!("{}", ans);
}