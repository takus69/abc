use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut ans = 0;
    let mut l = 0;
    let mut sum = 0;

    for r in 0..n {
        sum += a[r];
        while sum > k {
            sum -= a[l];
            l += 1;
        }
        ans += r-l+1;

    }
    println!("{}", ans);
}