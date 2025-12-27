use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
    }
    let mut s: Vec<usize> = s.iter().map(|&x| x.to_digit(10).unwrap() as usize).collect();
    let mut t: Vec<usize> = t.iter().map(|&x| x.to_digit(10).unwrap() as usize).collect();
    let mut ans = usize::MAX;
    for i in 0..(n-m+1) {
        let mut tmp = 0;
        for j in 0..m {
            tmp += (10+s[i+j] - t[j])%10;
        }
        ans = ans.min(tmp);
    }
    println!("{}", ans);
}