use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        xyz: [(usize, usize, usize); m],
    }

    let mut state = 0;
    let mut base = 1;
    for &ai in a.iter() {
        state += base*ai;
        base <<= 1;
    }

    let max_state = 2_usize.pow(n as u32);
    // println!("state: {}, max_state: {}", state, max_state);
    let mut dp: Vec<Vec<usize>> = vec![vec![usize::MAX; max_state]];
    dp[0][state] = 0;
    let mut pre_dp = dp[0].clone();
    for i in 0..m {
        let mut next_dp = pre_dp.clone();
        let (x, y, z) = xyz[i];
        for mut state in 0..max_state {
            let cnt = pre_dp[state];
            if cnt == usize::MAX { continue; }
            if (state >> (x-1)) & 1 == 1 {
                state -= 1 << (x-1);
            } else {
                state += 1 << (x-1);
            }
            if (state >> (y-1)) & 1 == 1 {
                state -= 1 << (y-1);
            } else {
                state += 1 << (y-1);
            }
            if (state >> (z-1)) & 1 == 1 {
                state -= 1 << (z-1);
            } else {
                state += 1 << (z-1);
            }
            next_dp[state] = next_dp[state].min(cnt+1);
        }
        pre_dp = next_dp;
    }

    // println!("dp: {:?}", dp);
    if pre_dp[max_state-1] == usize::MAX {
        println!("-1");
    } else {
        println!("{}", pre_dp[max_state-1]);
    }
}