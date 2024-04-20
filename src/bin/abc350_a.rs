use proconio::input;

fn main() {
    input! {
        S: String,
    }
    let n = S[3..].parse().unwrap();
    let mut ans = "No";
    if 1 <= n && n <= 349 && n != 316 {
        ans = "Yes";
    }
    println!("{}", ans);
}