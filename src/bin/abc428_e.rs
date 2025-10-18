use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n-1],
    }
    let mut edge: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    for &(a, b) in ab.iter() {
        edge[a].push(b);
        edge[b].push(a);
    }
    for i in 1..=n {
        edge[i].sort();
        edge[i].reverse();
    }

    fn bfs(s: usize, n: usize, edge: &Vec<Vec<usize>>) -> Vec<usize> {
        let mut dist: Vec<usize> = vec![0; n+1];
        let mut que: VecDeque<usize> = VecDeque::new();
        let mut visited: Vec<bool> = vec![false; n+1];
        que.push_front(s);
        visited[s] = true;
        while let Some(a) = que.pop_back() {
            let d = dist[a];
            for &b in edge[a].iter() {
                if visited[b] { continue; }
                que.push_front(b);
                visited[b] = true;
                dist[b] = d+1;
            }
        }

        dist
    }

    let dist = bfs(1, n, &edge);
    let mut max_dist = 0;
    let mut max_i1 = 0;
    for (i, &d) in dist.iter().enumerate().rev() {
        if max_dist < d {
            max_dist = d;
            max_i1 = i;
        }
    }
    let dist1 = bfs(max_i1, n, &edge);
    let mut max_dist = 0;
    let mut max_i2 = 0;
    for (i, &d) in dist1.iter().enumerate().rev() {
        if max_dist < d {
            max_dist = d;
            max_i2 = i;
        }
    }
    let dist2 = bfs(max_i2, n, &edge);
    for i in 1..=n {
        let ans = if dist1[i] > dist2[i] {
            max_i1
        } else if dist1[i] < dist2[i] {
            max_i2
        } else {
            max_i1.max(max_i2)
        };
        println!("{}", ans);
    }
}