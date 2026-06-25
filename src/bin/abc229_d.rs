use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        s: Chars,
        k: usize,
    }
    let mut que: VecDeque<usize> = VecDeque::new();
    let mut l: usize = 0;
    let mut ans = 0;
    for i in 0..s.len() {
        if s[i] == '.' {
            que.push_back(i);
            if que.len() > k {
                let j = que.pop_front().unwrap();
                l = j+1;
            }
        }
        ans = ans.max(i-l+1);
    }
    println!("{}", ans);
}