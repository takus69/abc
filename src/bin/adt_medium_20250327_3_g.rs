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
        n: usize,
    }
    let m = 998244353;
    let k = n.to_string().len();
    let mut ans = (n % m);
    let tmp = modint(10, n, m);
    let tmp = modint(tmp, k, m);
    ans *= (tmp + m -1) % m;
    ans %= m;
    let tmp = modint(10, k, m);
    let tmp = (tmp + m - 1) % m;
    let tmp = modinv(tmp, m);
    ans *= tmp;
    ans %= m;
    println!("{}", ans);

}