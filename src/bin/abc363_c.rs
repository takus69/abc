use proconio::{input, marker::Chars};
use std::collections::{HashSet, HashMap};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }
    let ss = s.clone().into_iter().collect::<HashSet<char>>();
    if ss.len() == n {
        let mut ans = 1;
        for i in 1..=n {
            ans *= i;
        }
        println!("{}", ans);
        std::process::exit(0);
    }
    let mut set: HashSet<Vec<char>> = HashSet::new();
    for perm in (0..n).permutations(n) {
        let mut tmp: Vec<char> = Vec::new();
        for i in perm.iter() {
            tmp.push(s[*i]);
        }
        set.insert(tmp);
    }
    let mut ans = set.len();
    for si in set.iter() {
        let mut flg = false;
        for i in 0..=(n-k) {
            let mut flg2 = true;
            for j in 1..=k {
                if si[i+j-1] != si[i+k-j] {
                    flg2 = false;
                    break;
                } 
            }
            if flg2 {
                flg = true;
                break;
            }
        }
        if flg { ans -= 1; }
    }
    println!("{}", ans);
}
 