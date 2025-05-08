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
        let mut pre = s[perm[0]].clone();
        for &i in perm.iter().skip(1) {
            let si = &s[i];

            let mut diff = 0;
            for j in 0..m {
                if pre[j] != si[j] {
                    diff += 1;
                }
            }
            if diff == 0 || diff > 1 {
                flg = false;
                break;
            }

            pre = si.clone();
        }
        if flg {
            println!("Yes");
            return;
        }
    }

    println!("No");
}