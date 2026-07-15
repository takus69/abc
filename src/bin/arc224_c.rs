use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        t: usize,
    }

    fn dfs(u: usize, edge: &[Vec<usize>], visited: &mut [bool], ans: &mut [usize]) {
        let mut max = 0;
        for &v in &edge[u] {
            if visited[v] {
                max = max.max(ans[v]);
            }
        }
        if u > 1 {
            visited[u] = true;
            ans[u] = max+1;
        }
        // println!("dfs: u: {}, max: {}", u, max);
        for &v in &edge[u] {
            if visited[v] { continue; }
            // println!("next dfs v: {}", v);
            dfs(v, edge, visited, ans);
        }
    }

    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            uv: [(usize, usize); m],
        }
        let mut edge: Vec<Vec<usize>> = vec![Vec::new(); n+1];
        for &(u, v) in &uv {
            edge[u].push(v);
            edge[v].push(u);
        }
        let mut visited: Vec<bool> = vec![false; n+1];
        let mut ans: Vec<usize> = vec![0; n+1];
        visited[1] = true;
        dfs(1, &edge, &mut visited, &mut ans);
        println!("{}", ans.iter().skip(1).join(" "));
    }
}