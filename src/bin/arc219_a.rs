use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }
    let mut target: Vec<usize> = (0..n).into_iter().collect();
    let mut ans: Vec<char> = Vec::new();
    for i in 0..m {
        let mut cnt0 = 0;
        for &j in target.iter() {
            if s[j][i] == '0' {
                cnt0 += 1;
            }
        }
        let c = if cnt0 > target.len()/2 { '0' } else { '1' };
        ans.push(c);
        let mut tmp: Vec<usize> = Vec::new();
        for &j in target.iter() {
            if s[j][i] != c {
                tmp.push(j);
            }
        }
        target = tmp;
    }
    if target.is_empty() {
        println!("Yes");
        println!("{}", ans.iter().join(""));
    } else {
        println!("No");
    }
}