use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        k: usize,
        pq: [(usize, usize); n],
    }
    let mut ans = 0;
    for (pi, qi) in pq.iter() {
        ans += pi * qi;
    }
    if ans < s {
        ans += k;
    }
    println!("{}", ans);
}