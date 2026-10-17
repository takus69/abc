use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: [Chars; 2],
    }
    if s[0][0] == 'D' { s[0][0] = 'A'; }
    if s[0][1] == 'D' { s[0][1] = 'A'; }
    if s[1][0] == 'D' { s[1][0] = 'A'; }
    if s[1][1] == 'D' { s[1][1] = 'A'; }
    if s[0][0] == s[1][0] || s[0][0] == s[0][1] || s[1][0] == s[1][1] || s[0][1] == s[1][1] {
        println!("Yes");
    } else {
        println!("No");
    }
}