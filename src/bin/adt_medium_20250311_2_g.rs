use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n-1],
        v: [usize; k],
    }
    let start = v[0];
    let set: HashSet<_> = v.into_iter().collect();
    let mut link: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    for &(ai, bi) in ab.iter() {
        link[ai].push(bi);
        link[bi].push(ai);
    }

    fn dfs(u: usize, set: &HashSet<usize>, link: &Vec<Vec<usize>>, visited: &mut Vec<bool>) -> (bool, usize) {
        visited[u] = true;
        let (mut flg, mut cnt) = if set.contains(&u) { (true, 1) } else { (false, 1) };

        for &v in link[u].iter() {
            if visited[v] { continue; }
            let (flg2, cnt2) = dfs(v, set, link, visited);
            flg |= flg2;
            if flg2 {
                cnt += cnt2;
            }
        }

        (flg, cnt)
    }

    let mut visited: Vec<bool> = vec![false; n+1];
    let (_, ans) = dfs(start, &set, &link, &mut visited);
    println!("{}", ans);
}