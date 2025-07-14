use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        uv: [(usize, usize); m],
    }

    let mut link: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    for &(u, v) in uv.iter() {
        link[u].push(v);
        link[v].push(u);
    }

    let mut r#mod = 998244353;
    let mut dp: Vec<Vec<usize>> = vec![vec![usize::MAX; n+1]];
    dp[0][1] = 1;
    for i in 0..k {
        let mut sum = 0;
        for &c in dp[i].iter() {
            if c != usize::MAX {
                sum += c;
                sum %= r#mod;
            }
        }

        dp.push(Vec::new());
        dp[i+1].push(usize::MAX);
        for j in 1..(n+1) {
            let mut del = if dp[i][j] != usize::MAX { dp[i][j] } else { 0 };
            for &l in link[j].iter() {
                if dp[i][l] == usize::MAX { continue; }
                del += dp[i][l];
                del %= r#mod;
            }
            let cnt = (r#mod+sum-del)%r#mod;
            if cnt != 0 {
                dp[i+1].push(cnt);
            } else {
                dp[i+1].push(usize::MAX);
            }
        }
    }
    let ans = if dp[k][1] != usize::MAX { dp[k][1] } else { 0 };
    println!("{}", ans);
}
