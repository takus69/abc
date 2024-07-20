use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: String,
        t: String,
    }
    let mut ans = 3;
    if s == t[0..n] {
        ans = 1;
        if s == t[(m-n)..m] {
            ans = 0;
        }
    } else if s == t[(m-n)..m] {
        ans = 2;
    }
    println!("{}", ans);
}