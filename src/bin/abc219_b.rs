use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [String; 3],
        t: Chars,
    }
    let mut ans = String::new();
    for &ti in &t {
        let i = ti as usize - '1' as usize;
        ans.push_str(&s[i]);
    }
    println!("{}", ans);
}