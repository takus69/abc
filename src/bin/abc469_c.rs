use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut cnt = 0;
    for &si in &s {
        if si=='o' {
            cnt += 1;
        }
    }
    let mut ans: Vec<usize> = Vec::new();
    for i in (0..n).rev() {
        while cnt > 0 && i+cnt < n && s[i+cnt] == 'o' {
            cnt -= 1;
        }
        let k= i+1;
        ans.push((k+cnt).min(n));
    }
    ans.reverse();
    for &a in &ans {
        println!("{}", a);
    }
}