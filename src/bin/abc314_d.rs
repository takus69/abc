use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
        txc: [(usize, usize, char); q],
    }
    let mut change = 0;
    let mut change_i = 0;
    for i in 0..q {
        let (t, x, c) = txc[i];
        if t == 1 {
            s[x-1] = c;
        } else {
            change = t;
            change_i = i;
        }
    }
    for i in 0..n {
        if change == 0 { break; }
        if change == 2 {
            s[i] = s[i].to_ascii_lowercase();
        } else {
            s[i] = s[i].to_ascii_uppercase();
        }
    }
    for i in (change_i+1)..q {
        let (t, x, c) = txc[i];
        if t == 1 {
            s[x-1] = c;
        }
    }
    println!("{}", s.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
}