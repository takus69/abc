use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        d: usize,
        lr: [(usize, usize); d],
    }
    let mut l_max: Vec<usize> = vec![0; n+1];
    let mut r_max: Vec<usize> = vec![0; n+1];
    for (i, &ai) in a.iter().enumerate() {
        l_max[i+1] = l_max[i].max(ai);
    }
    for (i, &ai) in a.iter().rev().enumerate() {
        r_max[i+1] = r_max[i].max(ai);
    }
    r_max.reverse();
    for &(l, r) in lr.iter() {
        println!("{}", l_max[l-1].max(r_max[r]));
    }
}