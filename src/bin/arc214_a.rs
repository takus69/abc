use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
    }
    let mut ks: Vec<char> = vec!['0'; 2*n-1];
    for k in 0..(2*n-1) {
        let st = if k < n { 0 } else { k-(n-1) };
        let t = (k+1).min(n);
        let mut tmp = '?';
        for i in st..t {
            let j = k-i;
            // println!("k: {}, i: {}, j: {}", k, i, j);
            if tmp == '?' {
                tmp = s[i][j];
                if s[i][j] != '?' {
                    ks[k] = s[i][j];
                }
            } else {
                if s[i][j] != '?' && s[i][j] != tmp {
                    println!("-1");
                    return;
                } else {
                    s[i][j] = tmp;
                }
            }
        }
    }
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '?' {
                s[i][j] = ks[i+j];
            }
        }
    }

    for si in s.iter() {
        println!("{}", si.iter().join(""));
    }
}