use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        t: usize,
    }

    // e: (to, from)
    fn dfs(graph: &[Vec<(usize, usize)>], e: (usize, usize), d: usize, history: &mut Vec<usize>, visited: &mut [bool], finished: &mut [bool], dist: &mut [usize]) -> (usize, usize) {
        visited[e.0] = true;
        history.push(e.0);
        dist[e.0] = d;
        
        for &e2 in &graph[e.0] {
            if e2.0 == e.1 { continue; }
            if finished[e2.0] { continue; }
            // println!("e: {:?} => e2: {:?}", e, e2);
            if visited[e2.0] && !finished[e2.0] {
                if (dist[e2.1]-dist[e2.0])%2 == 0 {
                    return e2;
                } else {
                    continue;
                }
            }
            let pos = dfs(graph, e2, d+1, history, visited, finished, dist);
            if pos.0 != 0 { return pos; }
        }
        finished[e.0] = true;
        history.pop();
        (0, 0)
    }

    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            ab: [(usize, usize); m],
        }
        let mut graph: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n+1];
        for &(a, b) in &ab {
            graph[a].push((b, a));
            graph[b].push((a, b));
        }
        let mut visited: Vec<bool> = vec![false; n+1];
        let mut finished: Vec<bool> = vec![false; n+1];
        let mut history: Vec<usize> = Vec::new();
        let mut dist: Vec<usize> = vec![0; n+1];
        let mut e = (0, 0);
        for i in 1..=n {
            if visited[i] { continue; }
            e = dfs(&graph, (i, 0), 1, &mut history, &mut visited, &mut finished, &mut dist);
            if e.0 != 0 {
                break;
            }

        }
        if history.is_empty() {
            println!("-1");
        } else {
            let mut ans: Vec<usize> = Vec::new();
            let mut flg = false;
            for &i in &history {
                if i == e.0 {
                    flg = true;
                }
                if flg {
                    ans.push(i);
                }
                if i == e.1 {
                    break;
                }
            }
            println!("{}", ans.len());
            println!("{}", ans.iter().join(" "));
        }
    }
}