use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        d: [usize; n*(n-1)/2],
    }
    let mut link: HashMap<(usize, usize), usize> = HashMap::new();
    let mut ij = 0;
    for i in 0..(n-1) {
        for j in (i+1)..n {
            link.insert((i, j), d[ij]);
            link.insert((j, i), d[ij]);
            ij += 1;
        }
    }
    let mut dp: Vec<usize> = vec![0; 2_usize.pow(n as u32)];
    for i in 0..(2_usize.pow(n as u32)) {
        for j in 0..n {
            let jj = 1 << j;
            if i & jj != 0 { continue; }
            for k in (j+1)..n {
                let kk = 1 << k;
                if i & kk != 0 { continue; }
                dp[i+jj+kk] = dp[i+jj+kk].max(dp[i] + link[&(j, k)]);
            }
        }
    }

    println!("{}", dp.iter().max().unwrap());
}