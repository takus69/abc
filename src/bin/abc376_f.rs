use proconio::input;

const INF: usize = 10_000_000;

fn main() {
    input! {
        n: usize,
        q: usize,
        ht: [(char, usize); q],
    }
    let mut dp: Vec<usize> = vec![INF; n+1];
    let mut pre_h = 'L';  // 0手目はLを動かしたことにする
    let mut pre_t = 1;  // Lは位置1
    dp[2] = 0;  // 0手目はLを動かしたからRの位置を保持
    for i in 0..q {
        // println!("i: {}, dp: {:?}", i, dp);
        let (h, t) = ht[i];
        // 今回動かす手と逆の手の状態数を保持
        let mut next_dp: Vec<usize> = vec![INF; n+1];
        // 前の動かした手と逆の手の状態数でiter
        for (i, &cnt) in dp.iter().enumerate() {
            if cnt == INF { continue; }
            let (now, now2) = if h == pre_h { (pre_t, i) } else { (i, pre_t) };  // (今回動かす手の位置, 逆の手の位置)
            let conv_now = 0;  // 今回動かす手を0にして考える
            let conv_now2 = (n+now2-now)%n;
            let conv_t = (n+t-now)%n;
            // println!("conv_now: {}, conv_now2: {}, conv_t: {}", conv_now, conv_now2, conv_t);
            if conv_now2 == conv_t {
                let (next, next2, cnt2) = (t, t%n+1, conv_t+1);
                next_dp[next2] = next_dp[next2].min(cnt+cnt2);
                let (next, next2, cnt2) = (t, (n+t-2)%n+1, n-conv_t+1);
                next_dp[next2] = next_dp[next2].min(cnt+cnt2);
            } else if conv_now2 > conv_t {
                let (next, next2, cnt2) = (t, now2, conv_t);
                next_dp[next2] = next_dp[next2].min(cnt+cnt2);
                let (next, next2, cnt2) = (t, (n+t-2)%n+1, (n-conv_t)+(conv_now2-conv_t+1));
                next_dp[next2] = next_dp[next2].min(cnt+cnt2);
            } else {
                let (next, next2, cnt2) = (t, t%n+1, conv_t+(conv_t-conv_now2+1));
                next_dp[next2] = next_dp[next2].min(cnt+cnt2);
                let (next, next2, cnt2) = (t, now2, n-conv_t);
                next_dp[next2] = next_dp[next2].min(cnt+cnt2);
            }
        }
        
        dp = next_dp;
        pre_h = h;
        pre_t = t;
    }

    // println!("dp: {:?}", dp);
    println!("{}", dp.iter().min().unwrap());

}