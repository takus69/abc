use proconio::input;
use itertools::Itertools;
use std::collections::{VecDeque, HashMap};

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m],
    }
    let mut link: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    for i in 1..=n {
        link.insert(i, vec![]);
    }
    for (ai, bi, ci) in abc.iter() {
        link.get_mut(ai).unwrap().push((*bi, *ci));
        link.get_mut(bi).unwrap().push((*ai, *ci));
    }
    fn dfs(start: usize, end: usize, visited: Vec<bool>, link: &HashMap<usize, Vec<(usize, usize)>>) -> (usize, usize) {
        if start == end { return (end, 0); }
        let mut ret: (usize, usize) = (start, 0);
        for (b, c) in link.get(&start).unwrap().iter() {
            if visited[*b] { continue; }
            let mut visited2 = visited.clone();
            visited2[*b] = true;
            let (e, d) = dfs(*b, end, visited2, link);
            if e == end && ret.1 <= c + d {
                ret = (e, c + d);
            }
        }

        ret
    }
    let mut ans = 0;
    for perm in (1..=n).permutations(2) {
        let start = perm[0];
        let end = perm[1];
        let mut visited = vec![false; n+1];
        visited[start] = true;
        let (e, d) = dfs(start, end, visited, &link);  // e!=endは到達不可能、e==endならdがその距離
        // println!("start: {}, end: {}, e: {}, d: {}", start, end, e, d);
        if e == end {
            ans = ans.max(d);
        }
    } 
    println!("{}", ans);
}