use proconio::input;

fn main() {
    input! {
        n: isize,
    }
    let p = 998244353;
    let mut ans = n % p;
    if ans < 0 {
        ans += p;
    }
    println!("{}", ans);
}