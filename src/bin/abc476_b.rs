use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }
    let mut ans = true;
    for i in 0..n {
        if t[i] == '*' { continue; }
        if t[i] != s[i] {
            ans = false;
            break;
        }
    }
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}