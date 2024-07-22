use proconio::input;

fn main() {
    input! {
        r: usize,
    }
    let mut ans = 300 - r;
    if r < 100 {
        ans = 100 - r;
    } else if r < 200 {
        ans = 200 - r;
    }
    println!("{}", ans);
}