use proconio::input;
use std::collections::HashMap;

pub fn primes(n: usize) -> Vec<usize> {
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
        n: usize,
    }
    let ps = primes(n);
    let mut sqrt: Vec<usize> = Vec::new();
    for &p in ps.iter() {
        if p*p <= n {
            sqrt.push(p*p);
        }
    }
    let mut map: HashMap<usize, usize> = HashMap::new();
    for i in 1..=n {
        let mut i = i;
        for &s in sqrt.iter() {
            while i%s == 0 {
                i /= s;
            }
        }
        let e = map.entry(i).or_insert(0);
        *e += 1;
    }

    let mut ans = n;
    for (_, &v) in map.iter() {
        if v > 1 {
            ans += v*(v-1);
        }
    }
    
    println!("{}", ans);
}