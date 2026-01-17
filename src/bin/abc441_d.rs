use proconio::input;
use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        s: usize,
        t: usize,
        uvc: [(usize, usize, usize); m],
    }
    let mut graph: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n+1];
    for &(u, v, c) in &uvc {
        graph[u].push((v, c));
    }

    fn dfs(u: usize, l_cnt: usize, cost: usize, ans: &mut HashSet<usize>, l: usize, s: usize, t: usize, graph: &Vec<Vec<(usize, usize)>>) {
        if l_cnt > l || cost > t { return; }
        if l_cnt == l && s <= cost && cost <= t {
            ans.insert(u);
            return;
        }
        for &(v, c) in graph[u].iter() {
            dfs(v, l_cnt+1, cost+c, ans, l, s, t, graph);
        }
    }

    let mut ans: HashSet<usize> = HashSet::new();
    dfs(1, 0, 0, &mut ans, l, s, t, &graph);

    let mut ans: Vec<usize> = ans.into_iter().collect();
    ans.sort();
    println!("{}", ans.iter().join(" "));
}