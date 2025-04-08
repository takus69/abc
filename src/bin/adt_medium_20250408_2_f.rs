use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
    }
    let mut cumsum: Vec<usize> = vec![0];
    let mut cnt = 0;
    for i in 0..(n-1) {
        if s[i] == s[i+1] {
            cnt += 1;
        }
        cumsum.push(cnt);
    }
    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }
        println!("{}", cumsum[r-1]-cumsum[l-1]);
    }
}