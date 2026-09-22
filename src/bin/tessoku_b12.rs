use proconio::input;

fn main() {
    input! {
        n: f64,
    }
    let mut ok: f64 = 0.0;
    let mut ng: f64 = f64::MAX;
    while ok+0.001 < ng {
        let m = (ok + ng) / 2.0;
        let f = m*m*m + m;
        if f < n {
            ok = m;
        } else {
            ng = m;
        }
    }
    println!("{}", ok);
}