use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut ans = n - n%5;
    if n%5 > 2 {
        ans += 5;
    }
    println!("{}", ans)
}