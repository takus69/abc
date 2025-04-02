use proconio::input;
use std::collections::{VecDeque, BinaryHeap};

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m],
    }
    let mut link: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n+1];
    for &(a, b, c) in abc.iter() {
        link[a].push((b, c));
        link[b].push((a, c));
    }
    let mut ans = 0;
    for s in 1..=n {
        let mut visited = vec![false; n+1];
        visited[s] = true;
        ans = ans.max(dfs(s, &visited, &link));
    }
    println!("{}", ans);

    fn dfs(a: usize, visited: &Vec<bool>, link: &Vec<Vec<(usize, usize)>>) -> usize {
        let mut ret = 0;
        for &(b, c) in link[a].iter() {
            if visited[b] { continue; }
            let mut v= visited.clone();
            v[b] = true;
            ret = ret.max(c + dfs(b, &v, &link));
        }

        ret
    }
}