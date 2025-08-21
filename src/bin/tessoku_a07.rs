use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(usize, usize); n],
    }
    let mut a: Vec<isize> = vec![0; d+1];
    for &(l, r) in lr.iter() {
        a[l-1] += 1;
        a[r] -= 1;
    }
    let mut cumsum: Vec<isize> = vec![0];
    for &ai in a.iter() {
        let last = cumsum.last().unwrap();
        cumsum.push(last+ai);
    }
    for i in 0..d {
        println!("{}", cumsum[i+1]);
    }
}