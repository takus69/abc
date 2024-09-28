use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let c = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>();
    let mut ans = 0;
    let mut pre = 0;
    let mut now = 0;
    for &ci in c.iter() {
        for j in 0..s.len() {
            if s[j] == ci {
                now = j;
                break;
            }
        }
        // println!("ci: {}, now: {}, pre: {}, ans: {}", ci, now, pre, ans);
        if ci == 'A' {
            pre = now;
            continue;
        }
        ans += now.abs_diff(pre);
        
        pre = now;
    }
    println!("{}", ans);
}