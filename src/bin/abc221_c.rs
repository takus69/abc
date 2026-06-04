use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        s: Chars,
    }
    let mut ans = 0;
    let n = s.len();
    for i in 1..n {
        for perm in (0..n).permutations(i) {
            let mut n1: Vec<char> = Vec::new();
            let mut n2: Vec<char> = Vec::new();
            for j in 0..n {
                if perm.contains(&j) {
                    n1.push(s[j]);
                } else {
                    n2.push(s[j]);
                }
            }
            n1.sort();
            n2.sort();
            n1.reverse();
            n2.reverse();
            if n1[0] == '0' || n2[0] == '0' { continue; }
            let n1: usize = n1.iter().collect::<String>().parse().unwrap();
            let n2: usize = n2.iter().collect::<String>().parse().unwrap();
            ans = ans.max(n1*n2);
        }
    }
    println!("{}", ans);
}