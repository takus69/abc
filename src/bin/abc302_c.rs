use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }
    for perm in (0..n).permutations(n) {
        let mut flg = true;
        for i in 0..(n-1) {
            let s1 = s[perm[i]].clone();
            let s2 = s[perm[i+1]].clone();
            let mut cnt = 0;
            for j in 0..m {
                if s1[j] != s2[j] {
                    cnt += 1;
                }
            }
            if cnt != 1 {
                flg = false;
                break;
            }
        }
        if flg {
            println!("Yes");
            std::process::exit(0);
        }
    }
    println!("No");
}