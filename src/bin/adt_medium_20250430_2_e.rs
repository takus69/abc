use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let mut c: Vec<(usize, usize)> = Vec::new();
    for (i, &ai) in a.iter().enumerate() {
        c.push((ai, i));
    }
    for (i, &bi) in b.iter().enumerate() {
        c.push((bi, i+n));
    }
    c.sort();
    let mut ans_a: Vec<usize> = Vec::new();
    let mut ans_b: Vec<usize> = Vec::new();
    for (j, &(ci, i)) in c.iter().enumerate() {
        if i < n {
            ans_a.push(j+1);
        } else {
            ans_b.push(j+1);
        }
    }
    println!("{}", ans_a.iter().join(" "));
    println!("{}", ans_b.iter().join(" "));
}