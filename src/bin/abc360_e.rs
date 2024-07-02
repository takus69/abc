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
        mut n: i64,
        k: i64,
    }

    let r#mod = 998244353;

    let m = modinv(n as usize, r#mod as usize);

    let mut x = 1;

    for _ in 0..k {
        x = (((n-2)*x + n+1)%r#mod) * m as i64;
        x %= r#mod;
    }
    println!("{}", x);
}