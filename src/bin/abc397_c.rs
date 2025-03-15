use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut head: HashSet<usize> = HashSet::new();
    let mut tail: HashSet<usize> = HashSet::new();
    let mut head_v: Vec<usize> = Vec::new();
    let mut tail_v: Vec<usize> = Vec::new();
    for &ai in a.iter() {
        head.insert(ai);
        head_v.push(head.len());
    }
    head_v.pop();
    for &ai in a.iter().rev() {
        tail.insert(ai);
        tail_v.push(tail.len());
    }
    tail_v.pop();
    tail_v.reverse();

    let mut ans = 0;
    for i in 0..(n-1) {
        ans = ans.max(head_v[i] + tail_v[i]);
    }
    println!("{}", ans);
}