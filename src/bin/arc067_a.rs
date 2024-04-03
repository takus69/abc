use proconio::input;
use std::collections::HashMap;

fn primes(n: usize) -> Vec<usize> {
    // エラトステネスの篩にて、n以下の素数リストを作成
    let mut is_prime: Vec<bool> = vec![true; n+1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=n {
        for j in ((i*2)..=n).step_by(i) {
            is_prime[j] = false;
        }
    }
    (0..=n).filter(|&i| is_prime[i]).collect()
}

fn main() {
    input! {
        n: i64,
    }

    const MOD: usize = 10_usize.pow(9_u32)+7;
    let mut p_cnt: HashMap<usize, usize> = HashMap::new();

    for mut i in 2..=n {
        for p in primes(n as usize) {
            while i % p as i64 == 0 {
                let cnt = p_cnt.get(&p).unwrap_or(&0);
                p_cnt.insert(p, cnt+1);
                i /= p as i64;
            }
        }
    }

    let mut ans = 1;
    for p in primes(n as usize) {
        ans = ((p_cnt.get(&p).unwrap_or(&0)+1) * ans) % MOD;
    }
    // println!("primes: {:?}, p_cnt: {:?}", primes(n as usize), p_cnt);
    println!("{}", ans);
}