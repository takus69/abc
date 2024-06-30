use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut ans = "No";
    if s[0] == 'R' || (s[1] == 'R' && s[2] == 'M') {
        ans = "Yes";
    }

    println!("{}", ans);
}