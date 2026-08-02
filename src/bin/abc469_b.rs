use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
    }
    let mut ans = 0;
    s.push('x');
    s.reverse();
    s.push('x');
    for i in 1..=n {
        if s[i] == 'x' && s[i-1] == 'x' && s[i+1] == 'x' {
            ans += 1;
        }
    }
    println!("{}", ans);
}