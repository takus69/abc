use proconio::{input, marker::Chars};
use std::collections::{HashMap, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        c: [usize; n],
    }
    let mut map: HashMap<usize, VecDeque<char>> = HashMap::new();
    for i in 0..n {
        let si = s[i];
        let ci = c[i];
        let e = map.entry(ci).or_insert(VecDeque::new());
        e.push_back(si);
    }
    for (_, mut que) in map.iter_mut() {
        let si = que.pop_back().unwrap();
        que.push_front(si);
    }
    let mut ans: Vec<String> = Vec::new();
    for ci in c.iter() {
        ans.push(map.get_mut(ci).unwrap().pop_front().unwrap().to_string());
    }
    println!("{}", ans.join(""));
}