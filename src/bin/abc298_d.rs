use proconio::input;
use std::collections::VecDeque;

fn modint(x: usize, n: usize, r#mod: usize) -> usize {
    // modを取りながら繰り返し二乗法
    let mut ret = 1;
    let mut x = x;
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            ret *= x;
            ret %= r#mod;
        }
        x *= x;
        x %= r#mod;
        n >>= 1;
    }
    ret
}

fn main() {
    input! {
        q: usize,
    }

    const MOD: usize = 998244353;
    let mut queue = VecDeque::new();
    queue.push_back(1);
    let mut ans = 1;

    for _ in 0..q {
        input! {
            q1: usize,
        }
        if q1 == 1 {
            input! {
                x: usize,
            }
            queue.push_back(x);
            ans = (ans * 10 + x) % MOD;
        } else if q1 == 2 {
            let x = queue.pop_front().unwrap();
            ans += MOD;
            ans -= (modint(10, queue.len(), MOD) * x) % MOD;
            ans %= MOD;
        } else {
            println!("{}", ans);
        }
    }
}
