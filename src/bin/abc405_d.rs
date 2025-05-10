use proconio::{input, marker::Chars};
use std::collections::{VecDeque, HashMap};
use itertools::Itertools;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let mut starts: VecDeque<(usize, usize)> = VecDeque::new();
    let mut cnt: Vec<Vec<usize>> = vec![vec![usize::MAX; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'E' {
                starts.push_front((i, j));
                cnt[i][j] = 0;
            }
        }
    }

    fn bfs(starts: &mut VecDeque<(usize, usize)>, ans: &mut Vec<Vec<char>>, cnt: &mut Vec<Vec<usize>>, s: &Vec<Vec<char>>) {
        let h = s.len() as i32;
        let w = s[0].len() as i32;
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let dirs_c = ['^', 'v', '<', '>'];
        let mut que: VecDeque<(usize, usize)> = VecDeque::new();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; w as usize]; h as usize];
        while let Some((si, sj)) = starts.pop_back() {
            que.push_front((si, sj));
            visited[si][sj] = true;
        }
        
        while let Some((i, j)) = que.pop_back() {
            let c = cnt[i][j];
            for dir_i in 0..4 {
                let (di, dj) = dirs[dir_i];
                let i2 = i as i32 + di;
                let j2 = j as i32 + dj;
                if i2 < 0 || i2 >= h || j2 < 0 || j2 >= w { continue; }
                let i2 = i2 as usize;
                let j2 = j2 as usize;
                if visited[i2][j2] || s[i2][j2] == '#' || s[i2][j2] == 'E' { continue; }
                visited[i2][j2] = true;
                if c+1 > cnt[i2][j2] { continue; }
                que.push_front((i2, j2));
                cnt[i2][j2] = c+1;
                ans[i2][j2] = dirs_c[dir_i];
            }
        }
    }

    let mut ans = s.clone();
    bfs(&mut starts, &mut ans, &mut cnt, &s);
    
    for a in ans.iter() {
        println!("{}", a.iter().join(""));
    }
}