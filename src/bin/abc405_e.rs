use proconio::input;

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

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    fn comb(a: usize, b: usize, memo: &Vec<usize>) -> usize {
        let r#mod = 998244353;
        let mut ret = 1;
        ret *= memo[a];
        ret %= r#mod;

        ret *= modinv(memo[b], r#mod);
        ret %= r#mod;
        ret *= modinv(memo[a-b], r#mod);
        ret %= r#mod;

        ret
    }

    let r#mod = 998244353;
    let mut ans = 0; 
    let mut val = 1;
    let mut memo: Vec<usize> = vec![val, val];
    for i in 2..10_000_000 {
        val *= i;
        val %= r#mod;
        memo.push(val);
    }
    for i in 0..=c {
        ans += comb(a+b+i, b, &memo) * comb(d+c-i-1, d-1, &memo);
        ans %= r#mod;
    }

    println!("{}", ans);
}