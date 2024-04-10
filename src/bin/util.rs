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

fn pow(x: usize, n: usize) -> usize {
    // 繰り返し二乗法
    let mut ret = 1;
    let mut x = x;
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            ret *= x;
        }
        x *= x;
        n >>= 1;
    }
    ret
}

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
    let msg = "test for primes";
    assert_eq!(primes(6), vec![2, 3, 5], "{}", msg);
    assert_eq!(primes(20), vec![2, 3, 5, 7, 11, 13, 17, 19], "{}", msg);

    let msg = "test for pow";
    assert_eq!(pow(3, 2), 9, "{}", msg);

    let msg = "test for modint";
    const MOD: usize = 998244353;
    assert_eq!(modint(10, 9, MOD), 10usize.pow(9) % MOD, "{}", msg);
}