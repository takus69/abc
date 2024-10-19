use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab: [(usize, usize); m],
    }
    let mut map: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..m {
        let (mut a, mut b) = ab[i];
        a -= 1;
        b -= 1;
        ab[i] = (a, b);
        map[a].push(b);
    }

    let mut visited: Vec<bool> = vec![false; n];
    let mut cnt: Vec<usize> = vec![0; n];
    let mut que: VecDeque<usize> = VecDeque::new();
    que.push_front(0);
    visited[0] = true;
    while !que.is_empty() {
        let now = que.pop_back().unwrap();
        let c = cnt[now];
        for &next in map[now].iter() {
            if next == 0 {
                println!("{}", c+1);
                std::process::exit(0);
            }
            if visited[next] { continue; }
            visited[next] = true;
            que.push_front(next);
            cnt[next] = c+1;
        }
    }
    println!("-1");

}