use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(isize, isize); n],
    }
    let mut fwk: Vec<Vec<isize>> = vec![vec![isize::MIN; w+1]; w+1];
    let mut d: Vec<BinaryHeap<isize>> = vec![BinaryHeap::new(); w+1];
    for &(wi, vi) in &wv {
        d[wi as usize].push(vi-1);
    }
    fwk[0][0] = 0;
    for wi in 1..=w {
        fwk[wi][0] = 0;
        for k in 1..=(w/wi) {
            if d[wi].is_empty() { continue; }
            let di = d[wi].pop().unwrap();
            fwk[wi][k] = fwk[wi][k-1] + di;
            // println!("wi: {}, k: {}, fwk: {}, {}, di: {}",wi, k, fwk[wi][k], fwk[wi][k-1], di);
            d[wi].push(di-2);
        }
    }
    // println!("d: {:?}", d);
    // println!("fwk: {:?}", fwk);

    let mut dp: Vec<Vec<isize>> = vec![vec![isize::MIN; w+1]; w+1];
    dp[0][0] = 0;
    for wi in 1..=w {
        dp[wi][0] = 0;
        for j in 0..=w {
            for k in 0..=(j/wi) {
                // println!("wi: {}, j: {}, dp: {}, fwk: {}", wi, j, dp[wi-1][j-k*wi], fwk[wi][k]);
                if dp[wi-1][j-k*wi] == isize::MIN || fwk[wi][k] == isize::MIN { continue; }
                dp[wi][j] = dp[wi][j].max(dp[wi-1][j-k*wi] + fwk[wi][k]);
            }
        }
    }
    // println!("dp: {:?}", dp);

    println!("{}", dp[w].iter().max().unwrap());
}