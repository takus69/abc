use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        s: Chars,
    }
    // Manacherのアルゴリズム
    let mut s2: Vec<char> = vec!['$'];
    for &si in &s {
        s2.push(si);
        s2.push('$');
    }
    let mut i = 0;
    let mut j = 0;
    let mut r: Vec<usize> = vec![1; s2.len()];
    while i < s2.len() {
        while i >= j && i+j < s2.len() && s2[i-j]==s2[i+j] { j += 1; }
        r[i] = j;
        let mut k = 1;
        while i >= k && k+r[i-k] < j { r[i+k] = r[i-k]; k += 1; }
        i += k; j -= k;
    }
    let n = s.len();
    let mut k = 0;
    for i in 0..n {
        // s[k..n]が回文か？
        if k+r[n+i] >= n { break; }
        k += 1;
    }
    let mut s2 = s.clone();
    s2.reverse();
    println!("{}{}", s.iter().join(""), s2[(n-k)..n].iter().join(""))
}