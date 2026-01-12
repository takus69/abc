use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut link: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    for &(a, b) in &ab {
        link[a].push(b);
        link[b].push(a);
    }
    
    let mut visited: Vec<bool> = vec![false; n+1];
    visited[1] = true;
    let cnt = dfs(1, 1, &mut visited, &link);
    if cnt == n {
        println!("The graph is connected.");
    } else {
        println!("The graph is not connected.");
    }
}

fn dfs(i: usize, cnt: usize, visited: &mut Vec<bool>, link: &Vec<Vec<usize>>) -> usize {
    let mut cnt = cnt;
    for &j in &link[i] {
        if visited[j] { continue; }
        visited[j] = true;
        cnt = dfs(j, cnt+1, visited, link);
    }

    cnt
}
