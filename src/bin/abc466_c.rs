use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut ans = 0;
    let mut l = 1;
    let mut r = 2;
    while l < n && r <= n {
        println!("? {} {}", l, r);
        input! {
            s: String,
        }
        if s == "Yes" {
            r += 1;
            if r > n {
                ans += r-l-1;
                l += 1;
                r = n;
            }
        } else {
            ans += r-l-1;
            l += 1;
            if r == l { r += 1; }
        }
    }
    println!("! {}", ans);
}