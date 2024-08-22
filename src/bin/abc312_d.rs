use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let r#mod = 998244353;
    let mut bra_cnt = 0;
    let mut cket_cnt = 0;
    let mut q_cnt = 0;
    for si in s.iter() {
        match si {
            '?' => { q_cnt += 1; },
            '(' => { bra_cnt += 1; },
            ')' => { cket_cnt += 1; },
            _ => {},
        }
    }
    let max_cnt = s.len() / 2;
    if s.len() % 2 == 1 || bra_cnt > max_cnt || cket_cnt > max_cnt {
        println!("0");
        std::process::exit(0);
    }
    let mut dp: Vec<Vec<usize>> = Vec::new();  // i個目の括弧までで、j個の(がある組合せの数
    dp.push(vec![0; 1502]);
    dp[0][0] = 1;
    for i in 0..s.len() {
        dp.push(vec![0; 1502]);
        let si = s[i];
        for j in 0..1501 {
            if j > i+1 { break; }
            if si == '(' {
                if j > 0 && j <= max_cnt {
                    dp[i+1][j] += dp[i][j-1];
                }
            } else if si == ')' {
                if j >= i+1 - j {
                    dp[i+1][j] += dp[i][j];
                }
            } else {  // ?の場合
                if j <= max_cnt && j > 0 {  // '('にする
                    dp[i+1][j] += dp[i][j-1];
                }
                if j >= i+1 - j {
                    dp[i+1][j] += dp[i][j];
                }
                dp[i+1][j] %= r#mod;
            }
        }
    }
    println!("{}", (dp[s.len()].iter().sum::<usize>())%r#mod);
}