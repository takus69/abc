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
        q: usize,
        x: [usize; q],
    }
    let p = primes(*x.iter().max().unwrap());
    let p: HashSet<usize> = p.into_iter().collect();
    for xi in x.iter() {
        if p.contains(xi) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}