use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }
    let mut ans = n;
    for perm in (0..n).permutations(n) {
        let mut tmp = 0;
        let mut pop: Vec<bool> = vec![false; m];
        for (i, &pi) in perm.iter().enumerate() {
            for j in 0..m {
                if s[pi][j] == 'o' && !pop[j] {
                    tmp += 1;
                    pop[j] = true;
                }
                if tmp == m {
                    ans = ans.min(i+1);
                    break;
                }
            }
        }
    }
    println!("{}", ans);
}