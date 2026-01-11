use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m], 
    }
    let mut link: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    for &(a, b) in ab.iter() {
        link[a].push(b);
        link[b].push(a);
    }
    for i in 1..=n {
        println!("{}: {{{}}}", i, link[i].iter().join(", "));
    }
}