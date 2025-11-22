use proconio::{input, marker::Chars};

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
        s: Chars,
    }

    let r#mod = 998244353;
    let mut fact: Vec<usize> = vec![1];
    let mut base = 1;
    for i in 1..=s.len() {
        base *= i;
        base %= r#mod;
        fact.push(base);
    }

    fn comb(a: usize, b: usize, r#mod: usize, fact: &Vec<usize>) -> usize {
        let mut ret = fact[a];
        ret *= modinv(fact[a-b], r#mod);
        ret %= r#mod;
        ret *= modinv(fact[b], r#mod);
        ret %= r#mod;

        ret
    }

    let mut cnt1: Vec<Vec<usize>> = Vec::new(); 
    let mut base: Vec<usize> = vec![0; 10];
    for &si in s.iter() {
        let n = si.to_digit(10).unwrap() as usize;
        base[n] += 1;
        cnt1.push(base.clone());
    }
    let mut cnt2: Vec<Vec<usize>> = Vec::new();
    let mut base: Vec<usize> = vec![0; 10];
    for &si in s.iter().rev() {
        let n = si.to_digit(10).unwrap() as usize;
        base[n] += 1;
        cnt2.push(base.clone());
    }
    cnt2.reverse();
    let mut ans = 0;
    for (i, &si) in s.iter().enumerate() {
        let n = si.to_digit(10).unwrap() as usize;
        if n == 9 || i+1 == s.len() { continue; }
        let a = cnt1[i][n];
        let b = cnt2[i+1][n+1];
        ans += comb(a+b, a, r#mod, &fact);
        ans %= r#mod;
        ans += r#mod - comb(a+b-1, a-1, r#mod, &fact);
        ans %= r#mod;
    }
    println!("{}", ans);
}