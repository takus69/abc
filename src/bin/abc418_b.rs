use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut ans = 0.0;
    for l in 0..s.len() {
        for r in (l+2)..s.len() {
            if s[l]!='t' || s[r]!='t' { continue; } 
            if r-l+1 < 3 { continue; }
            let mut x = 0;
            for i in l..=r {
                if s[i] == 't' {
                    x += 1;
                }
            }
            let tmp = (x as f64 - 2.0) / (r-l-1) as f64;
            if tmp > ans {
                ans = tmp
            }
        }
    }
    println!("{}", ans);
}