use proconio::input;
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    let ans = solver(n, a);
    println!("{}", ans);
}

fn solver(n: usize, mut a: Vec<i64>) -> usize {
    let r#mod: usize = 1000000007;
    a.insert(0, 0);
    let mut dp = vec![HashMap::new(); n+1];
    let mut next = vec![HashMap::new(); n+1];
    dp[0].insert(0, 1);
    next[0].insert(0, 1);
    let mut zero_idx = vec![0];
    let mut zero_cnt = vec![1];
    let mut zero_sum = 1;
    for i in 1..=n {
        let keys = next[i-1].keys().cloned().collect::<Vec<i64>>();
        for k in keys {
            // 一つ前が0でなく、i桁目を選択するパターン
            if k != 0 {
                let v = *(next[i-1].get(&k).unwrap());
                let cnt = dp[i].entry(a[i]+k).or_insert(0);
                *cnt += v;
                *cnt %= r#mod;
                let cnt = next[i].entry(a[i]+k).or_insert(0);
                *cnt += v;
                *cnt %= r#mod;
            } else {
                // 一つ前が0で、i桁目を選択するパターン
                for ll in 0..zero_idx.len() {
                    let l = zero_idx[ll];
                    let l_cnt = zero_cnt[ll];
                    let mut flg = true;
                    let mut uniq: HashSet<i64> = a[(l+1)..=i].to_vec().into_iter().collect();
                    uniq.insert(0);
                    for ai in uniq {
                        let cnt = next[i].entry(ai).or_insert(0);
                        *cnt += l_cnt;
                        *cnt %= r#mod;
                    }
                }
            }
            // println!("i: {}, k1: {}, next[i]: {:?}", i, k, next[i]);
        }
        // i桁目を選択しないパターン
        let keys = dp[i-1].keys().cloned().collect::<Vec<i64>>();
        for k in keys {
            let v = *(dp[i-1].get(&k).unwrap());
            let cnt = dp[i].entry(k).or_insert(0);
            *cnt += v;
            *cnt %= r#mod;
            if k != 0 {
                let cnt = next[i].entry(k).or_insert(0);
                *cnt += v;
                *cnt %= r#mod;
            }
            // println!("i: {}, k2: {}, next[i]: {:?}", i, k, next[i]);
        }
        
        let keys = next[i].keys().cloned().collect::<Vec<i64>>();
        for k in keys {
            if a[i]+k == 0 {
                let cnt = *(next[i].get(&0).unwrap());
                zero_idx.push(i);
                zero_cnt.push((cnt + r#mod - zero_sum)%r#mod);
                zero_sum = cnt;
            }
        }
    }
    let mut ans = 0;
    for (k, v) in dp[n].iter() {
        ans += v;
        ans %= r#mod;

    }
    /*
    println!("zero_idx: {:?}", zero_idx);
    println!("zero_cnt: {:?}", zero_cnt);
    println!("next: {:?}", next);
    println!("dp: {:?}", dp);
    */
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let n = 3;
        let a = vec![1, 1, 2];
        assert_eq!(solver(n, a), 4);
    }

    #[test]
    fn sample2() {
        let n = 4;
        let a = vec![1, -1, 1, -1];
        assert_eq!(solver(n, a), 8);
    }

    #[test]
    fn sample3() {
        let n = 5;
        let a = vec![0, 0, 0, 0, 0];
        assert_eq!(solver(n, a), 1);
    }

    #[test]
    fn sample4() {
        let n = 40;
        let a = vec![
            2, -2, 1, 3, -3, -1, -2, -3, 0, -1,
            -2, 0, -3, 0, 0, 2, 0, -1, 2, -2,
            -2, -1, 3, -2, -2, -2, 2, 3, 2, -3,
            0, -2, 2, 1, 3, 0, -1, 0, -2, -3,
        ];
        assert_eq!(solver(n, a), 420429545);
    }
}
