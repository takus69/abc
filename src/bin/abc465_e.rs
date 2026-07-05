use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }

    const MOD: usize = 998244353;
    let mut dp: Vec<Vec<Vec<Vec<usize>>>> = vec![vec![vec![vec![0, 0]; 3]; 1<<10]];  // i桁目, 使用済み数字の集合, 3で割ったあまり, nと等しい(1)か小さい(0)か
    dp[0][0][0][1] = 1;
    for (i, &d) in n.iter().enumerate() {  // 上の桁から順に処理
        let d = d.to_digit(10).unwrap() as usize;
        let mut next_dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![0, 0]; 3]; 1<<10];
        for j in 0..10 {  // 追加する数字
            for set in 0..(1<<10) {  // 使っている集合
                for m in 0..3 {  // 3で割った余り
                    for l in 0..2 {  // nと等しいか(1)小さいか(0)
                        if dp[i][set][m][l] == 0 { continue; }
                        if l==1 && j > d { continue; }
                        let next_set = if (set >> j)&1==0 && !(j==0&&set==0) {
                            set + (1<<j)
                        } else {
                            set
                        };
                        let next_m = (m+j)%3;
                        let next_l = if l==1 && j==d { 1 } else { 0 };
                        next_dp[next_set][next_m][next_l] += dp[i][set][m][l];
                        next_dp[next_set][next_m][next_l] %= MOD;
                        // println!("i: {}, d: {}, j: {}, set: {}, m: {}, l: {}, next_dp: {}, dp: {}", i, d, j, set, m, l, next_dp[next_set][next_m][next_l], dp[i][set][m][l]);
                    }
                }
            }
        }
        dp.push(next_dp);
    }

    let mut ans = 0;
    let N = n.len();
    for set in 1..(1<<10) {  // 使っている集合
        for m in 0..3 {  // 3で割った余り
            for l in 0..2 {  // nと等しいか(1)小さいか(0)
                let mut cnt = 0;
                for i in 0..10 {
                    if (set>>i)&1==1 { cnt += 1; }
                }
                let flg_cnt = cnt==3;
                let flg3 = (set>>3)&1==1;
                let flg_m = m==0;
                if (flg_cnt && !flg3 && !flg_m) || (!flg_cnt && flg3 && !flg_m) || (!flg_cnt && !flg3 && flg_m) {
                    ans += dp[N][set][m][l];
                    ans %= MOD;
                }
            }
        }
    }
    println!("{}", ans);
}