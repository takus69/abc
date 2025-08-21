use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        lr: [(usize, usize); q],
    }
    let mut cumsum: Vec<usize> = vec![0];
    for &ai in a.iter() {
        let last = cumsum.last().unwrap();
        cumsum.push(last+ai);
    }
    for &(l, r) in lr.iter() {
        println!("{}", cumsum[r] - cumsum[l-1]);
    }
}