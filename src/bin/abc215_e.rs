use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    // dp[i][j][k] i番目のコンテストに参加した際に、これまで参加したコンテストの状態j(=2^10で表現)であり、最後のコンテストがk
    const MOD: usize = 998244353;
    const M: usize = 2usize.pow(10);
    fn c2i(c: char) -> usize {
        (c as u8 - b'A') as usize
    }
    let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; 10]; M]];
    dp[0][0][c2i(s[0])] = 1;
    for (i, &si) in s.iter().enumerate() {
        let ci = c2i(si);
        let mut next_dp: Vec<Vec<usize>> = dp[i].clone();
        for (j, dpi) in dp[i].iter().enumerate() {
            for (k, &c) in dpi.iter().enumerate() {
                if ((j>>ci)&1)==1 {  // 参加済みのコンテスト
                    if k==ci {  // 最後のコンテストと一致
                        next_dp[j][ci] += c;  // 参加
                        next_dp[j][ci] %= MOD;
                    }
                } else {
                    next_dp[j | (1<<ci)][ci] += c;  // 参加
                    next_dp[j | (1<<ci)][ci] %= MOD;
                }
            }
        }
        dp.push(next_dp);
    }
    let mut ans = 0;
    for dpi in dp[n].iter().skip(1) {
        for c in dpi.iter() {
            ans += c;
            ans %= MOD;
        }
    }
    println!("{}", ans);
}