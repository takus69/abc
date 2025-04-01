use proconio::{input, marker::Chars};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: u32,
        s: [Chars; n],
    }
    fn r#move(i: u32, j: u32, di: i32, dj: i32, s: &Vec<Vec<char>>) -> (u32, u32) {
        let i = i as i32;
        let j = j as i32;
        let n = s.len() as i32;
        let mut i2 = i + di;
        let mut j2 = j + dj;
        if i2 < 0 || i2 >= n { i2 = i; }
        if j2 < 0 || j2 >= n { j2 = j; }
        let si = s[i2 as usize][j2 as usize];
        if si == '#' {
            i2 = i;
            j2 = j;
        }

        (i2 as u32, j2 as u32)
    }

    let mut players: Vec<(u32, u32)> = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if s[i as usize][j as usize] == 'P' {
                players.push((i, j));
            }
        }
    }
    let mut que: VecDeque<(u32, u32, u32, u32, u32)> = VecDeque::new();
    let mut visited: HashSet<(u32, u32, u32, u32)> = HashSet::new();
    que.push_front((players[0].0, players[0].1, players[1].0, players[1].1, 0));
    visited.insert((players[0].0, players[0].1, players[1].0, players[1].1));
    while let Some((i1, j1, i2, j2, cnt)) = que.pop_back() {
        for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (ii1, jj1) = r#move(i1, j1, di, dj, &s);
            let (ii2, jj2) = r#move(i2, j2, di, dj, &s);
            if i1 == ii1 && j1 == jj1 && i2 == ii2 && j2 == jj2 { continue; }  // 移動できない場合
            if ii1 == ii2 && jj1 == jj2 { println!("{}", cnt+1); return; }  // 同じ位置の場合
            if visited.contains(&(ii1, jj1, ii2, jj2)) { continue; }  // 訪問済みの場合
            que.push_front((ii1, jj1, ii2, jj2, cnt+1));
            visited.insert((ii1, jj1, ii2, jj2));
        }
    }
    println!("-1");
}