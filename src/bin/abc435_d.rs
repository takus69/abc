use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(usize, usize); m],
        q: usize,
    }
    let mut link: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    for &(x, y) in xy.iter() {
        link[y].push(x);
    }
    let mut black: Vec<bool> = vec![false; n+1];
    for _ in 0..q {
        input! {
            c: usize,
            v: usize,
        }
        match c {
            1 => {
                let mut que: VecDeque<usize> = VecDeque::new();
                que.push_front(v);
                while let Some(u) = que.pop_back() {
                    if black[u] { continue; }
                    black[u] = true;
                    for &u2 in link[u].iter() {
                        if black[u2] { continue; }
                        que.push_front(u2);
                    }
                }
            },
            2 => {
                if black[v] {
                    println!("Yes");
                } else {
                    println!("No");
                }
            },
            _ => {},
        }
    }
}