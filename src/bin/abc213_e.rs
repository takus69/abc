use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut s: [Chars; h],
    }

    // 出発地から到達可能なマスに対して始点BFS
    // 壁があったらパンチしてキューに追加
    // 各マスの到達までのパンチ数を持つ

    const NOT_VISITED: usize = usize::MAX;
    let mut visited: Vec<Vec<usize>> = vec![vec![NOT_VISITED; w]; h];
    let mut que: VecDeque<(usize, usize)> = VecDeque:: new();
    que.push_back((0, 0));
    visited[0][0] = 0;
    while let Some((i, j)) = que.pop_front() {
        let cnt = visited[i][j];
        bfs(i, j, &mut visited, &mut que, &s);  // 直接行けるところはqueに追加, cnt更新
        // 隣接を確認
        for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let i2 = i as i32 + di;
            let j2 = j as i32 + dj;
            if i2 < 0 || i2 >= h as i32 || j2 < 0 || j2 >= w as i32 { continue; }  // 区画外
            let i2 = i2 as usize;
            let j2 = j2 as usize;
            if s[i2][j2] == '#' {
                // パンチ
                for (di2, dj2) in punch_dir(di, dj) {
                    let i3 = i as i32 + di2;
                    let j3 = j as i32 + dj2;
                    if i3 < 0 || i3 >= h as i32 || j3 < 0 || j3 >= w as i32 { continue; }  // 区画外
                    let (i3, j3) = (i3 as usize, j3 as usize);
                    if visited[i3][j3] == NOT_VISITED && s[i3][j3] == '#' {
                        visited[i3][j3] = cnt+1;
                        que.push_back((i3, j3));
                    }
                }
            }
        }
    }

    fn punch_dir(di: i32, dj: i32) -> [(i32, i32); 6] {
        if di == 0 {
            [(di, dj), (di, dj*2), (di+1, dj), (di+1, dj*2), (di-1, dj), (di-1, dj*2)]
        } else {
            [(di, dj), (di*2, dj), (di, dj+1), (di*2, dj+1), (di, dj-1), (di*2, dj-1)]
        }
    }

    fn bfs(i: usize, j: usize, visited: &mut [Vec<usize>], que2: &mut VecDeque<(usize, usize)>, s: &[Vec<char>]) {
        let h = s.len() as i32;
        let w = s[0].len() as i32;
        let cnt = visited[i][j];
        let mut que: VecDeque<(usize, usize)> = VecDeque::new();
        que.push_front((i, j));
        while let Some((i, j)) = que.pop_back() {
            for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let i2 = i as i32 + di;
                let j2 = j as i32 + dj;
                if i2 < 0 || i2 >= h || j2 < 0 || j2 >= w { continue; }  // 区画外
                let i2 = i2 as usize;
                let j2 = j2 as usize;
                if visited[i2][j2] == NOT_VISITED && s[i2][j2] == '.' {
                    visited[i2][j2] = cnt;
                    que2.push_front((i2, j2));
                    que.push_back((i2, j2));
                }
            }
        }
    }

    println!("{}", visited[h-1][w-1]);
}