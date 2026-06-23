use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        k: usize,
    }
    let mut k_cnt = 0;
    let mut e_cnt = 0;
    let mut y_cnt = 0;
    for &si in &s {
        match si {
            'K' => { k_cnt += 1; },
            'E' => { e_cnt += 1; },
            'Y' => { y_cnt += 1; },
            _ => {},
        }
    }
    let n = s.len();
    let max_cnt = k.min(n*(n+1)/2);
    let mut dp: Vec<Vec<Vec<Vec<usize>>>> = vec![vec![vec![vec![0; max_cnt+1]; y_cnt+1]; e_cnt+1]; k_cnt+1];
    dp[0][0][0][0] = 1;
    for k_cnt2 in 0..=k_cnt {
        for e_cnt2 in 0..=e_cnt {
            for y_cnt2 in 0..=y_cnt {
                for x in 0..= max_cnt {
                    let i = k_cnt2+e_cnt2+y_cnt2;
                    if i >= n { continue; }
                    // Kを追加
                    if k_cnt2+1 <= k_cnt {
                        let mut next_cnt = next_cnt('K', k_cnt2, e_cnt2, y_cnt2, &s);
                        if x+next_cnt <= max_cnt {
                            dp[k_cnt2+1][e_cnt2][y_cnt2][x+next_cnt] += dp[k_cnt2][e_cnt2][y_cnt2][x];
                        }
                    }
                    // Eを追加
                    if e_cnt2+1 <= e_cnt {
                        let mut next_cnt = next_cnt('E', k_cnt2, e_cnt2, y_cnt2, &s);
                        if x+next_cnt <= max_cnt {
                            dp[k_cnt2][e_cnt2+1][y_cnt2][x+next_cnt] += dp[k_cnt2][e_cnt2][y_cnt2][x];
                        }
                    }
                    // Yを追加
                    if y_cnt2+1 <= y_cnt {
                        let mut next_cnt = next_cnt('Y', k_cnt2, e_cnt2, y_cnt2, &s);
                        if x+next_cnt <= max_cnt {
                            dp[k_cnt2][e_cnt2][y_cnt2+1][x+next_cnt] += dp[k_cnt2][e_cnt2][y_cnt2][x];
                        }
                    }
                }
            }
        }
    }

    let mut ans = 0;
    for &v in &dp[k_cnt][e_cnt][y_cnt] {
        ans += v;
    }
    /**
    for k_cnt2 in 0..=k_cnt {
        for e_cnt2 in 0..=e_cnt {
            for y_cnt2 in 0..=y_cnt {
                println!("dp: ({}, {}, {}), {:?}", k_cnt2, e_cnt2, y_cnt2, dp[k_cnt2][e_cnt2][y_cnt2]);
            }
        }
    }*/
    println!("{}", ans);

    fn next_cnt(c: char, k: usize, e: usize, y: usize, s: &[char]) -> usize {
        let mut next_cnt = 0;
        let mut k_cnt = 0;
        let mut e_cnt = 0;
        let mut y_cnt = 0;
        for &si in s {
            match si {
                'K' => {
                    k_cnt += 1;
                    if k_cnt > k { next_cnt += 1; }
                },
                'E' => {
                    e_cnt += 1;
                    if e_cnt > e { next_cnt += 1; }
                },
                'Y' => {
                    y_cnt += 1;
                    if y_cnt > y { next_cnt += 1; }
                },
                _ => {},
            }
            if si == c && (c=='K'&&k_cnt>k || c=='E'&&e_cnt>e || c=='Y'&&y_cnt>y) {
                break;
            }
        }
        
        next_cnt-1
    }
}