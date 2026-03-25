use proconio::input;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n],
    }
    ab.sort_by(|a, b| (a.0, Reverse(a.1)).cmp(&(b.0, Reverse(b.1))));
    // println!("{:?}", ab);
    // aでソートすると、bの最長増加部分列を求めるだけ
    let mut l: Vec<usize> = vec![usize::MAX];
    for &(_, b) in &ab {
        if l.last().unwrap() < &b  {
            l.push(b);
        } else {
            let idx = l.partition_point(|&x| x < b);
            l[idx] = b;
        }
    }
    // println!("{:?}", l);
    println!("{}", l.len());
}