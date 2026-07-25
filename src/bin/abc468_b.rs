use proconio::{input, marker::Chars};

fn main() {
    input! {
        m: usize,
        d: usize,
        s: Chars,
    }
    let mut check: Vec<bool> = vec![false; m];
    for i in 0..m {
        if s[i] == 'G' {
            let start = if i < d { 0 } else { i - d };
            let end = (i+d).min(m-1);
            for j in start..=end {
                check[j] = true;
            }
        }
    }
    let mut ans = 0;
    for i in 0..m {
        if !check[i] { ans += 1; }
    }
    println!("{}", ans);
}