use proconio::input;

fn main() {
    input! {
        n: usize,
        mut lr: [(usize, usize); n],
    }
    lr.sort_by(|a, b| a.1.cmp(&b.1));
    let mut ans = 0;
    let mut last = 0;
    for &(li, ri) in lr.iter() {
        if last <= li {
            ans += 1;
            last = ri;
        }
    }
    println!("{}", ans);
}