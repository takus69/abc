use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; n]; m],
    }
    let bit = 2_usize.pow(n as u32);
    let mut dp: Vec<Vec<usize>> = vec![vec![usize::MAX; bit]];  // dp[i][j] i枚目までで使う使わないのbit(j)で、何枚使うか
    dp[0][0] = 0;
    for i in 0..m {
        dp.push(dp[i].clone());
        for j in 0..bit {
            if dp[i][j] == usize::MAX { continue; }
            let mut tmp = j;
            for (k, &ai)  in a[i].iter().enumerate() {
                if ai == 1 && (j>>k)%2 == 0 {
                    tmp += 1 << k;
                }
            }
            dp[i+1][tmp] = dp[i+1][tmp].min(dp[i][j] + 1);
        }
    }

    println!("{}", if dp[m][bit-1]==usize::MAX { -1 } else { dp[m][bit-1] as isize });
}