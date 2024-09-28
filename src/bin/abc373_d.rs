use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        uvw: [(usize, usize, i64); m],
    }
    let mut link: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for (u, v, w) in uvw.iter() {
        link[u-1].push((v-1, *w));
        link[v-1].push((u-1, -w));
    }

    let mut visited = vec![false; n];
    let mut que: VecDeque<usize> = VecDeque::new();
    let mut ans: Vec<i64> = vec![0; n];
    for i in 0..n {
        if visited[i] { continue; }
        que.push_front(i);

        while !que.is_empty() {
            let u = que.pop_back().unwrap();
            for (v, w) in link[u].iter() {
                if visited[*v] { continue; }
                ans[*v] = ans[u] as i64 + w;
                visited[*v] = true;
                que.push_front(*v);
            }
        }
    }
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}