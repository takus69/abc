use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut ans = 0;
    let mut tmp = 0;
    for &si in &s {
        if si == 'R' {
            tmp += 1;
            ans = ans.max(tmp);
        } else {
            tmp = 0;
        }
    }
    println!("{}", ans);
}