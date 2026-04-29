use proconio::input;
use std::collections::{HashMap, HashSet};
use ac_library::Dsu;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m],
        k: usize,
        t: usize,
        d: [usize; k],
        q: usize,
    }

    fn update(a: usize, b:usize, c: usize, t: usize, path: &mut [Vec<usize>], ans: &mut [usize]) {
        let before = path[a][b];
        if before > 0 && before <= c { return; }
        let c2 = if b == 0 { c - t } else { c };
        path[a][b] = c;
        path[b][a] = c2;
        if before == 0 {
            ans[a] += c;
            ans[b] += c2;
        } else {
            ans[a] -= before - c;
            ans[b] -= before - c2;
        }
    }

    fn add_path(a: usize, b: usize, c: usize, dist: &mut [Vec<usize>]) {
        let n = dist.len()-1;
        dist[a][b] = dist[a][b].min(c);
        dist[b][a] = dist[b][a].min(c);

        for i in 0..=n {
            for j in 0..=n {
                if dist[i][a] != usize::MAX && dist[b][j] != usize::MAX {
                    dist[i][j] = dist[i][j].min(dist[i][a] + dist[a][b] + dist[b][j]);
                    // dist[j][i] = dist[j][i].min(dist[j][b] + dist[b][a] + dist[a][j]);
                }
                if dist[i][b] != usize::MAX && dist[a][j] != usize::MAX {
                    dist[i][j] = dist[i][j].min(dist[i][b] + dist[b][a] + dist[a][j]);
                    // dist[j][i] = dist[j][i].min(dist[j][a] + dist[a][b] + dist[b][j]);
                }
            }
        }
    }

    fn add_airport(a: usize, t: usize, dist: &mut [Vec<usize>]) {
        let n = dist.len()-1;
        let b = 0;
        dist[a][b] = 0;
        dist[b][a] = t;

        for i in 0..=n {
            for j in 0..=n {
                if dist[i][a] != usize::MAX && dist[b][j] != usize::MAX {
                    dist[i][j] = dist[i][j].min(dist[i][a] + dist[a][b] + dist[b][j]);
                }
                if dist[i][b] != usize::MAX && dist[a][j] != usize::MAX {
                    dist[i][j] = dist[i][j].min(dist[i][b] + dist[b][a] + dist[a][j]);
                }
                if dist[j][b] != usize::MAX && dist[a][j] != usize::MAX {
                    // dist[j][i] = dist[j][i].min(dist[j][b] + dist[b][a] + dist[a][j]);
                }
                if dist[j][a] != usize::MAX && dist[b][i] != usize::MAX {
                    // dist[j][i] = dist[j][i].min(dist[j][a] + dist[a][b] + dist[b][i]);
                }
            }
        }
    }

    fn ans(dist: &[Vec<usize>]) -> usize {
        let n = dist.len() -1;
        let mut ans = 0;
        for i in 1..=n {
            for j in 1..=n {
                if dist[i][j] == usize::MAX { continue; }
                ans += dist[i][j];
            }
        }

        ans
    }

    let mut dist: Vec<Vec<usize>> = vec![vec![usize::MAX; n+1]; n+1];
    for i in 0..=n {
        dist[i][i] = 0;
    }

    for &(a, b, c) in abc.iter() {
        dist[a][b] = dist[a][b].min(c);
        dist[b][a] = dist[b][a].min(c);
    }

    for &a in d.iter() {
        dist[a][0] = 0;
        dist[0][a] = t;
    }

    for k in 0..=n {
        for i in 0..=n {
            for j in 0..=n {
                if dist[i][k] == usize::MAX || dist[k][j] == usize::MAX { continue; }
                let next = dist[i][k] + dist[k][j];
                dist[i][j] = dist[i][j].min(next);
                // dist[j][i] = dist[j][i].min(next);
            }
        }
    }

    for _ in 0..q {
        input! {
            cmd: usize,
        }
        match cmd {
            3 => {
                println!("{}", ans(&dist));
            },
            2 => {
                input! {
                    x: usize,
                }
                add_airport(x, t, &mut dist);
            },
            1 => {
                input! {
                    x: usize,
                    y: usize,
                    c: usize,
                }
                add_path(x, y, c, &mut dist);
            },
            _ => {},
        }
    }
}