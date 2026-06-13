use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut ans: String = String::new();
    for &si in &s {
        if si.is_ascii_digit() {
            ans.push(si);
        }
    }
    println!("{}", ans);
}