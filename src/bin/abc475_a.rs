use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut ans = String::new();
    for &si in &s {
        ans.push(si);
        ans.push('o');
    }
    ans.pop();
    println!("{}", ans);
}