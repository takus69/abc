use proconio::input;
use std::collections::HashMap;

fn dfs(s: usize, t: usize, graph: &Vec<HashMap<usize, usize>>, visited: &mut Vec<bool>) -> (usize, Vec<usize>) {
    visited[s] = true;
    let mut min_f = usize::MAX;
    let mut path: Vec<usize> = Vec::new();
    for (&i, &c) in graph[s].iter() {
        if visited[i] || c == 0 { continue; }
        if i == t {
            min_f = min_f.min(c);
            path.push(t);
            break;
        } else {
            let (f, mut path2) = dfs(i, t, graph, visited);
            if path2.is_empty() || path2[0] != t { continue; }
            min_f = min_f.min(f.min(c));
            path2.push(i);
            path = path2;
            break;
        }
    }

    (min_f, path)
}

fn update(graph: &mut Vec<HashMap<usize, usize>>, f: usize, path: Vec<usize>) {
    let mut from = *path.last().unwrap();
    for &to in path.iter().rev().skip(1) {
        let e = graph[from].get_mut(&to).unwrap();
        *e -= f;
        let e = graph[to].get_mut(&from).unwrap();
        *e += f;
        from = to;
    }

}

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m],
    }

    let mut graph: Vec<HashMap<usize, usize>> = vec![HashMap::new(); n+1];
    for &(a, b, c) in abc.iter() {
        let e = graph[a].entry(b).or_insert(0);
        *e = c;
        let e = graph[b].entry(a).or_insert(0);
    }

    loop {
        let mut visited: Vec<bool> = vec![false; n+1];
        let (f, mut path) = dfs(1, n, &graph, &mut visited);
        if path.is_empty() { break; }
        path.push(1);
        update(&mut graph, f, path);
    }
    let mut ans = 0;
    for (_, &c) in graph[n].iter() {
        ans += c;
    }
    for &(a, b, c) in abc.iter() {
        if a == n {
            ans -= c;
        }
    }
    println!("{}", ans);
}