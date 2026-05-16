use proconio::input;

pub fn pow(x: usize, n: usize) -> usize {
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

pub fn modint(x: usize, n: usize, r#mod: usize) -> usize {
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

/// 逆元
pub fn modinv(x: usize, r#mod: usize) -> usize {
    modint(x, r#mod-2, r#mod)
}

const MOD: usize = 998244353;

fn main() {
    input! {
        x1: usize,
        x2: usize,
        x3: usize,
    }

    let mut value = 1;
    let mut fact: Vec<usize> = vec![value];
    let mut inv: Vec<usize> = vec![modinv(value, MOD)];
    for i in 1..=(x1+x2+x3) {
        value *= i;
        value %= MOD;
        fact.push(value);
        inv.push(modinv(value, MOD));
    }

    let mut ans = 0;
    // 1..1
    for i in 2..=x1 {
        if x3 < i-1 || x2 < 2*i-2 { continue; }
        let mut tmp = 1;
        tmp *= comb(x1-1, i-1, &fact, &inv);
        tmp %= MOD;
        tmp *= comb(x3-1, i-2, &fact, &inv);
        tmp %= MOD;
        tmp *= comb(x1+x2+x3-(i*2-2), x1+x3, &fact, &inv);
        tmp %= MOD;
        ans += tmp;
        ans %= MOD;
    }

    // 3..3
    for i in 2..=x3 {
        if x1 < i-1 || x2 < 2*i-2 { continue; }
        let mut tmp = 1;
        tmp *= comb(x3-1, i-1, &fact, &inv);
        tmp %= MOD;
        tmp *= comb(x1-1, i-2, &fact, &inv);
        tmp %= MOD;
        tmp *= comb(x1+x2+x3-(i*2-2), x1+x3, &fact, &inv);
        tmp %= MOD;
        ans += tmp;
        ans %= MOD;
    }

    // 1..3, 3..1
    for i in 1..=x1.min(x3) {
        if x1 < i-1 || x3 < i-1 || x2 < 2*i-1 { continue; }
        let mut tmp = 1;
        tmp *= comb(x1-1, i-1, &fact, &inv);
        tmp %= MOD;
        tmp *= comb(x3-1, i-1, &fact, &inv);
        tmp %= MOD;
        tmp *= comb(x1+x2+x3-(i*2-1), x1+x3, &fact, &inv);
        tmp %= MOD;
        ans += 2*tmp;
        ans %= MOD;
    }

    fn comb(a: usize, b: usize, fact: &[usize], inv: &[usize]) -> usize {
        let mut tmp = 1;
        tmp *= fact[a];
        tmp *= inv[b];
        tmp %= MOD;
        tmp *= inv[a-b];
        tmp %= MOD;
        tmp
    }

    // println!("fact: {:?}, inv: {:?}", fact, inv);
    println!("{}", ans);
}