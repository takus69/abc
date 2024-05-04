use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let mut max_ba = 0;
    let mut ans = 0;
    for (a, b) in ab {
        ans += a;
        max_ba = max_ba.max(b - a);
    }
    ans += max_ba;
    println!("{}", ans);
}