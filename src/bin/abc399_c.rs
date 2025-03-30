use proconio::input;
use std::collections::{VecDeque, HashSet};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }
    let mut link: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    for &(u, v) in uv.iter() {
        link[u].push(v);
        link[v].push(u);
    }

    let mut ans= 0;
    let mut visited: Vec<usize> = vec![0; n+1];
    for i in 1..=n {
        if visited[i] > 0 { continue; }
        let mut que: VecDeque<(usize, usize)> = VecDeque::new();
        que.push_front((i, 0));
        visited[i] = 1;
        while let Some((u, pre)) = que.pop_front() {
            for &v in link[u].iter() {
                if v == pre { continue; }  // 元の遷移
                if visited[v] > 0 {
                    // println!("visited {} => {}", u, v);
                    visited[v] += 1;
                    continue;
                }
                que.push_front((v, u));
                visited[v] = 1;
            }
        }
    }
    for &cnt in visited.iter() {
        ans += if cnt > 0 { cnt - 1 } else { 0 };
    }
    // println!("visited: {:?}", visited);
    println!("{}", ans/2);
}