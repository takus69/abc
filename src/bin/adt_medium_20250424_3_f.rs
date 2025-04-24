use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        t: Chars,
        s: [Chars; n],
    }
    let mut ans: Vec<usize> = Vec::new();
    for (i, si) in s.iter().enumerate() {
        if t.len() == si.len() {
            let mut cnt = 0;
            for j in 0..t.len() {
                if t[j] != si[j] { cnt += 1; }
            }
            if cnt <= 1 {
                ans.push(i+1);
            }
        } else if t.len()+1 == si.len() {
            let mut cnt = 0;
            let mut k = 0;
            for j in 0..t.len() {
                if t[j] != si[j+k] {
                    cnt += 1;
                    k += 1;
                    if k > 1 || t[j] != si[j+k] {
                        cnt = usize::MAX;
                        break;
                    }
                }
            }
            if cnt <= 1 {
                ans.push(i+1);
            }
        } else if t.len() == si.len()+1 {
            let mut cnt = 0;
            let mut k = 0;
            for j in 0..si.len() {
                if t[j+k] != si[j] {
                    cnt += 1;
                    k += 1;
                    if k > 1 || t[j+k] != si[j] {
                        cnt = usize::MAX;
                        break;
                    }
                }
            }
            if cnt <= 1 {
                ans.push(i+1);
            }
        }
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}