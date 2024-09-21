use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut ans = "".to_string();
    for si in s.iter() {
        if si == &'.' { continue; }
        ans += &si.to_string();
    }
    println!("{}", ans);
}