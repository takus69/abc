use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        s: Chars,
    }
    let mut s: VecDeque<char> = s.into_iter().collect();
    let mut ss: Vec<VecDeque<char>> = vec![s.clone()];
    for _ in 0..s.len() {
        let c = s.pop_back().unwrap();
        s.push_front(c);
        ss.push(s.clone());
    }
    ss.sort();
    println!("{}", ss[0].iter().collect::<String>());
    println!("{}", ss.last().unwrap().iter().collect::<String>());
}