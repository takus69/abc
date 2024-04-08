use proconio::input;
use proconio::marker::Chars;
use std::collections::{VecDeque, HashMap};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    i: usize,
    j: usize,
}

fn bfs(start: &Pos, a: &[Vec<char>]) -> Vec<Vec<i64>> {
    let h = a.len();
    let w = a[0].len();
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; w]; h];
    let mut distance = vec![vec![-1; w]; h];
    distance[start.i][start.j] = 0;
    let dirs: [(i64, i64); 4] = [(1, 0), (-1, 0), (0, -1), (0, 1)];
    // 初期値
    queue.push_back((*start, 0));
    visited[start.i][start.j] = true;
    // bfs
    while let Some((pos, d)) = queue.pop_front() {
        for dir in dirs {
            let di = pos.i as i64 + dir.0;
            let dj = pos.j as i64 + dir.1;
            if di < 0 || di > h as i64 -1 {
                continue;
            }
            if dj < 0 || dj > w as i64 -1 {
                continue;
            }
            let pos = Pos{ i: di as usize, j: dj as usize };
            if visited[pos.i][pos.j] || a[pos.i][pos.j] == '#' { continue; }
            queue.push_back((pos, d+1));
            visited[pos.i][pos.j] = true;
            distance[pos.i][pos.j] = d+1;
        }
    }
    distance
}

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
        n: usize,
        rce: [(usize, usize, i64); n],
    }

    // 幅優先探索でゴールまでの最短距離と各位置の最大エネルギーを求める
    let mut potion = HashMap::new();
    let mut flg = HashMap::new();

    for (i, j, e) in &rce {
        let pos = Pos{ i: i-1, j: j-1 };
        potion.insert(pos, *e);
        flg.insert(pos, false);
    }
    // println!("potion: {:?}", potion);

    // 初期状態をキューに追加
    let mut start = Pos { i: 0, j: 0 };
    let mut goal = Pos { i: 0, j: 0 };
    for (i, _) in a.iter().enumerate() {
        for j in 0..w {
            if a[i][j] == 'S' {
                start = Pos{ i, j };
            } else if a[i][j] == 'T' {
                goal = Pos{ i, j }
            }
        }
    }

    // start位置の薬から順に探索
    let mut queue = VecDeque::new();
    if potion.contains_key(&start) {
        queue.push_back(&start);
        flg.insert(start, true);
    }
    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();
        let distance = bfs(pos, &a);
        // println!("pos: {:?}, distance: {:?}", pos, distance);
        let e = potion[&pos];
        // let d = distance[pos.i][pos.j];
        let d = distance[goal.i][goal.j];
        if e >= d && d > 0 {
            println!("Yes");
            std::process::exit(0);
        }
        for (j, (pos2, _)) in potion.iter().enumerate() {
            if flg[pos2] { continue; }
            let d2 = distance[pos2.i][pos2.j];
            // println!("pos2: {:?}, d2: {}, e: {}", pos2, d2, e);
            if d2 > e || d2 < 0 { continue; }
            queue.push_back(pos2);
            flg.insert(*pos2, true);
        }
    }
    println!("No");
}
