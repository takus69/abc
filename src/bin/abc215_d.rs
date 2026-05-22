use proconio::input;
use std::collections::HashSet;

pub fn primes(n: usize) -> Vec<usize> {
    // エラトステネスの篩にて、n以下の素数リストを作成
    // 計算量O(N log log N)
    let mut is_prime: Vec<bool> = vec![true; n+1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2.. {
        if i*i > n { break; }
        for j in ((i*i)..=n).step_by(i) {
            is_prime[j] = false;
        }
    }
    (0..=n).filter(|&i| is_prime[i]).collect()
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }
    // aに含まれる素数をすべて出す（√ai以下の素数のみチェック）
    // 含まれる素数をエラトステネスの篩の形で除外していく
    let ps = primes(m);
    let mut has_p: HashSet<usize> = HashSet::new();
    for &ai in &a {
        for &p in &ps {
            if p > ai { break; }
            if ai%p == 0 {
                has_p.insert(p);
            }
        }
    }
    let mut ans: Vec<bool> = vec![true; m+1];
    ans[0] = false;
    for &p in &has_p {
        for i in (p..=m).step_by(p) {
            ans[i] = false;
        }
    }
    let mut cnt = 0;
    for &a in &ans {
        if a { cnt+=1; }
    }
    println!("{}", cnt);
    for (i, &a) in ans.iter().enumerate() {
        if a { println!("{}", i); }
    }
}