use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        uv: [(usize, usize); n-1],
    }
    let mut link: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    for (u, v) in uv {
        link[u].push(v);
        link[v].push(u);
    }

    fn dfs(i: usize, a: &Vec<usize>, link: &Vec<Vec<usize>>, visited: &mut Vec<bool>, ans: &mut Vec<bool>, cnt: &mut HashMap<usize, usize>) {
        for &j in link[i].iter() {
            if visited[j] { continue; }
            visited[j] = true;
            let ai = a[j-1];
            let e = cnt.entry(ai).or_insert(0);
            *e += 1;
            if *e > 1 || ans[i] { ans[j] = true; }
            dfs(j, a, link, visited, ans, cnt);
            visited[j] = false;
            let e = cnt.entry(ai).or_insert(0);
            *e -= 1;
        }
    }

    let mut cnt: HashMap<usize, usize> = HashMap::new();
    cnt.insert(a[0], 1);
    let mut ans: Vec<bool> = vec![false; n+1];
    let mut visited: Vec<bool> = vec![false; n+1];
    visited[1] = true;
    dfs(1, &a, &link, &mut visited, &mut ans, &mut cnt);

    for &ai in ans.iter().skip(1) {
        if ai {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}