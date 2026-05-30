use proconio::input;

fn main() {
    input! {
        n: usize,
        mut m: usize,
    }
    let mut ans = 0;
    while m != 0 {
        m = n%m;
        ans += 1;
    }
    println!("{}", ans);
}