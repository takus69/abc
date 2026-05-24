use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let mut q: Vec<usize> = vec![0; n];
    for (i, &pi) in p.iter().enumerate() {
        q[pi-1] = i+1;
    }

    println!("{}", q.iter().join(" "));
}