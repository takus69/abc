use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
        s: Chars,
    }

    let mut que: VecDeque<(usize, usize)> = VecDeque::new();
    let mut dist1: Vec<(usize, usize)> = vec![(usize::MAX, usize::MAX); n];
    let mut dist2: Vec<(usize, usize)> = vec![(usize::MAX, usize::MAX); n];
    let mut visited: Vec<usize> = vec![0; n];
    let mut link: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut pre: Vec<usize> = vec![usize::MAX; n];
    for &(u, v) in uv.iter() {
        link[u-1].push(v-1);
        link[v-1].push(u-1);
    }
    for (i, &si) in s.iter().enumerate() {
        if si == 'S' {
            que.push_front((i, i));
            dist1[i] = (0, i);
            visited[i] += 1;
            pre[i] = i;
        }
    }
    while let Some((u, from)) = que.pop_back() {
        for &v in link[u].iter() {
            if v == from { continue; }
            // println!("u: {}, v: {}", u, v);
            // println!("visited: {:?}", visited);
            let (d, from) = if pre[v]==u { dist2[u] } else { dist1[u] };
            if d == usize::MAX { continue; }
            if visited[v] == 2 { continue; }
            if visited[v] == 0 {
                dist1[v] = (d+1, from);
                pre[v] = u;
                visited[v] += 1;
            } else {
                let (_, pre_from) = dist1[v];
                // if pre_from != from {
                    dist2[v] = (d+1, from);
                    visited[v] += 1;
                // }
            }
            que.push_front((v, u));
        }
        // println!("dist1: {:?}", dist1);
        // println!("dist2: {:?}", dist2);
    }
    for (i, &si) in s.iter().enumerate() {
        if si == 'D' {
            let (d1, _) = dist1[i];
            let (d2, _) = dist2[i];
            println!("{}", d1+d2);
        }
    }
}