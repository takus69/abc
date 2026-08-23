use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut ans: String = String::new();
    for &si in &s {
        if si == 'A' {
            ans.push(si);
        } else {
            ans.push('.');
        }
    }
    println!("{}", ans);
}