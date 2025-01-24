use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let max_a = ((n as f64).powf(1.0 / 3.0) + 1.0) as usize;
    let mut ans = 0;
    for a in 1..=max_a {
        let max_b = ((n as f64 / a as f64).powf(1.0/ 2.0) + 1.0) as usize;
        if a*a*a > n { break; }
        for b in a..=max_b {
            if a*b*b > n { break; }
            if (n /(a*b)) * b * a > n { break; }
            ans += (n / (a*b)) - b + 1;
        }

    }
    println!("{}", ans);
}