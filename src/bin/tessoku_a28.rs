use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut ans = 0;
    let mut r#mod = 10000;
    for _ in 0..n {
        input! {
            t: char,
            a: usize,
        }
        match t {
            '+' => { ans += a; },
            '-' => { ans += r#mod - a; },
            '*' => { ans *= a; },
            _ => {},
        }
        ans %= r#mod;
        println!("{}", ans);
    }
}