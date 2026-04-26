use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            h: usize,
            w: usize,
            s: [Chars; h],
        }

        let n = 2usize.pow(w as u32);
        let mut dp: Vec<Vec<isize>> = vec![vec![isize::MAX; n]];
        dp[0][0] = 0;
        for i in 0..h {
            let mut tmp_dp: Vec<isize> = vec![isize::MAX; n];
            for j in 0..n {
                if dp[i][j] == isize::MAX { continue; }
                let states = parse_states(&s[i]);
                for &(k, cnt) in states.iter() {
                    // println!("check j: {}, k: {}, check: {}", j, k, check(j, k, w));
                    if check(j, k, w) {
                        tmp_dp[k] = (dp[i][j]+cnt).min(tmp_dp[k]);
                    }
                }
            }
            dp.push(tmp_dp);
        }

        println!("{}", dp[h].iter().min().unwrap());

        fn parse_states(s: &[char]) -> Vec<(usize, isize)> {
            let mut ret: Vec<(usize, isize)> = Vec::new();
            let n = s.len() as u32;
            for i in 0..2usize.pow(n) {
                let mut cnt = 0;
                for j in 0..n {
                    let f = i>>j&1;
                    if f==1 && s[j as usize]=='.' {
                        cnt = isize::MAX;
                        break;
                    }
                    if f==0 && s[j as usize] == '#' {
                        cnt += 1;
                    }
                }
                if cnt == isize::MAX { continue; }
                ret.push((i, cnt));
            }

            ret
        }

        fn check(j: usize, k: usize, w: usize) -> bool {
            for i in 0..(w-1) {
                if j>>i&1==1 && k>>i&1==1 && j>>(i+1)&1==1 && k>>(i+1)&1==1 {
                    return false;
                }
            }

            true
        }
    }
}