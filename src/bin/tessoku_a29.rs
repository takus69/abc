use proconio::input;

fn main() {
    input! {
        a: usize,
        mut b: usize,
    }
    let r#mod = 1000000007;
    let mut ans = 1;
    let mut tmp = a;
    while b > 0 {
        if b % 2 == 1 {
            ans *= tmp;
            ans %= r#mod;
        }
        tmp *= tmp;
        tmp %= r#mod;
        b /= 2;
    }
    println!("{}", ans);
}