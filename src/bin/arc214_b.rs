use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            abc: [(usize, usize, usize); m],
        }
        if n % 2 == 1 {
            println!("-1");
            continue;
        }
        let mut link: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n+1];
        for &(a, b, c) in abc.iter() {
            link[a].push((b, c));
            link[b].push((a, c));
        }
        let mut ans = 0;
        for i in 1..=n {
            ans ^= i;
        }
        let mut que: VecDeque<(usize, usize)> = VecDeque::new();
        let mut visited: Vec<bool> = vec![false; n+1];
        que.push_front((1, 0));
        visited[1] = true;
        while let Some((a, c)) = que.pop_back() {
            for &(b, c2) in link[a].iter() {
                if visited[b] { continue; }
                que.push_front((b, c^c2));
                ans ^= c^c2;
                visited[b] = true;
            }
        }
        println!("{}", ans);
    }
}