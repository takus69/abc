use proconio::{input, marker::Chars};
use std::collections::{VecDeque, HashMap, HashSet};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let mut que: VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mut map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] != '.' && s[i][j] != '#' {
                let e = map.entry(s[i][j]).or_insert(Vec::new());
                e.push((i, j));
            }
        }
    }
    let mut ans = -1;
    let mut visited: Vec<Vec<bool>> = vec![vec![false; w]; h];
    let mut set: HashSet<char> = HashSet::new();
    que.push_front((0, 0, 0));
    visited[0][0] = true;
    while let Some((i, j, d)) = que.pop_back() {
        if (i, j) == (h-1, w-1) { ans = d as isize; break; }
        for (di, dj) in [(0, -1), (0, 1), (1, 0), (-1, 0)] {
            let i2 = i as isize + di;
            let j2 = j as isize + dj;
            if i2 < 0 || i2 >= h as isize { continue; }
            if j2 < 0 || j2 >= w as isize { continue; }
            let i2 = i2 as usize;
            let j2 = j2 as usize;
            if visited[i2][j2] || s[i2][j2] == '#' { continue; }
            que.push_front((i2, j2, d+1));
            visited[i2][j2] = true;
        }

        if s[i][j] != '.' && !set.contains(&s[i][j]) {
            set.insert(s[i][j]);
            for &(i2, j2) in map.get(&s[i][j]).unwrap().iter() {
                if visited[i2][j2] { continue; }
                que.push_front((i2, j2, d+1));
                visited[i2][j2] = true;
            }
        }
}

    println!("{}", ans);
}