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

/// 以下2つの解法
/// 1. 繰り返し二乗法(コンテスト中にACした解法)
/// 2. 等比数列の和(乗法の逆元)
fn main() {
    input! {
        mut n: usize,
    }
    let mut cnt = 1;
    let mut nn = n;
    while nn > 0 {
        nn /= 10;
        if nn > 0 {
            cnt += 1;
        }
    }
    let r#mod = 998244353;
    let mut ans = 0;
    let mut i = 1;
    let mut pre_i = 0;
    let mut nn = n % r#mod;
    while n > 0 {
        // println!("start n: {}, pre_i: {}, cnt: {}", n, pre_i, cnt);
        if n & 1 == 1 {
            ans += nn * modint(10, pre_i*cnt, r#mod);
            ans %= r#mod;
            pre_i += i;
        }
        nn += (nn % r#mod) * modint(10, i*cnt, r#mod);
        nn %= r#mod;
        i <<= 1;
        n >>= 1;
        // println!("n: {}, ans: {}, nn: {}", n, ans, nn);
    }
    println!("{}", ans);

    // 等比数列の和
    // let mut ans = (n % r#mod) * (modint(10, cnt*n, r#mod)) * modint(10, cnt, r#mod))

}