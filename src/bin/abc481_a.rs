use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let mut ans = 0;
    let mut n = a+b;
    while n > 0 {
        ans += 1;
        n /= 10;
    }

    println!("{}", ans);
}