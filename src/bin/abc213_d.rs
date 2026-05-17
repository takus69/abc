use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
    }
    let mut edge: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    for _ in 0..n-1 {
        input! {
            a: usize,
            b: usize,
        }
        edge[a].push(b);
        edge[b].push(a);
    }
    for i in 1..=n {
        edge[i].sort();
    }

    let mut visited: Vec<bool> = vec![false; n+1];
    let mut ans: Vec<usize> = Vec::new();
    dfs(1, &mut edge, &mut visited, &mut ans);
    println!("{}", ans.iter().join(" "));

    fn dfs(i: usize, edge: &mut Vec<Vec<usize>>, visited: &mut Vec<bool>, ans: &mut Vec<usize>) {
        ans.push(i);
        visited[i] = true;

        let e = edge[i].clone();
        for &j in e.iter() {
            if visited[j] { continue; }
            dfs(j, edge, visited, ans);
            ans.push(i);
        }
    }
}