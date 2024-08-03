use proconio::input;

fn main() {
    input! {
        n: usize,
        m: i64,
        a: [i64; n],
    }
    let inf = 1<<60;
    let mut lo = 0;
    let mut hi = inf;
    while lo + 1 < hi {
        let mut sum_a = 0;
        let x = (lo + hi) / 2;
        for ai in a.iter() {
            sum_a += x.min(*ai);
        }
        // println!("hi: {}, x: {}, lo: {}, m: {}, sum_a: {}", hi, x, lo, m, sum_a);
        if sum_a <= m {
            lo = x;
        } else {
            hi = x;
        }
    }
    if hi == inf {
        println!("infinite");
    } else {
        println!("{}", lo);
    }
}