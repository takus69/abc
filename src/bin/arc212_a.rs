use proconio::input;

fn main() {
    input! {
        k: usize,
    }
    let r#mod = 998244353;
    let mut ans: usize = 0;
    for a in 2..=(k-4) {
        for b in 2..=(k-a-2) {
            let c = k-a-b;
            ans += (a-1)*(b-1)*(c-1)*(k - a.max(b).max(c));
            ans %= r#mod;
        }
    }
    println!("{}", ans);
}