use proconio::input;
use std::collections::{VecDeque, HashSet};

fn main() {
    input! {
        n: usize,
        x: [usize; n],
    }
    let mut ans = 0;
    let mut que: VecDeque<usize> = VecDeque::new();
    let mut set: HashSet<usize> = HashSet::new();
    let mut i = 0;
    while i < n-1 {
        if que.is_empty() {
            while i < n-2 && x[i+1] == x[i+2] {
                i += 1;
            }
        }
        if x[i] == x[i+1] {
            if set.contains(&x[i]) {
                while set.contains(&x[i]) {
                    let q = que.pop_front().unwrap();
                    set.remove(&q);
                }
            }
            que.push_back(x[i]);
            set.insert(x[i]);
            ans = ans.max(que.len()*2);
            i += 2;
        } else {
            if i > 0 && x[i-1] == x[i] {
                i -= 1;
            } else {
                i += 1;
            }
            que = VecDeque::new();
            set = HashSet::new();
        }
    }
    println!("{}", ans);
}