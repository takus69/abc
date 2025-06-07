use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            s: Chars,
        }
        let mut ans: Vec<char> = vec![s[0]];
        let mut now = s[0];
        let mut start = true;
        let mut end = false;
        let mut r#move = s[0];
        for i in 1..s.len() {
            if start {
                if now > s[i] {
                    r#move = now;
                    start = false;
                    end = true;
                    ans.pop();
                }
                ans.push(s[i]);
                now = s[i];
            } else {
                if r#move < s[i] && end {
                    ans.push(r#move);
                    end = false;
                }
                ans.push(s[i]);
            }
        }
        if ans.len() < n {
            ans.push(r#move);
        }
        println!("{}", ans.iter().join(""));
    }
}