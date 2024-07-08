use proconio::{input, marker::Chars};
use std::collections::{HashMap, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n]
    }

    // ビット全探索(mask: どの売り場に行くか)
    let mut ans = n;
    let goal = vec!["o"; m].join("");
    for mask in 0..(2_i32.pow(n as u32)) {  // どの売り場に行くか
        let mut tmp = vec!["x"; m];
        let mut cnt = 0;
        for i in 0..n {
            if (mask >> i) & 1 == 1 {  // 行く売り場が決まる
                cnt += 1;
                for j in 0..m {  // どのポップコーンが食べられるか
                    let c = &s[i][j];
                    if c == &'o' {
                        tmp[j] = "o";
                    }
                }
            }
        }
        let tmp = tmp.join("");
        if tmp == goal {
            ans = ans.min(cnt);
        }
    }
    println!("{}", ans);
}