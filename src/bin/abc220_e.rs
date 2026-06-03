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

fn main() {
    input! {
        n: usize,
        d: usize,
    }
    /**
    // 愚直解
    let mut ans = 0;
    for i in 1..2usize.pow(n as u32) {
        for j in 1..2usize.pow(n as u32) {
            let mut cnt = 0;
            let mut i2 = i;
            let mut j2 = j;
            while i2 != j2 {
                if i2 > j2 {
                    i2 /= 2;
                    cnt += 1;
                } else {
                    j2 /= 2;
                    cnt += 1;
                }
            }
            if cnt == d {
                ans += 1;
                // println!("(i, j)=({}, {})", i, j);
            }
        }
    }
    println!("ans: {}", ans);
    */

    const MOD: usize = 998244353;
    let mut base: u128 = 1;
    let pow_d: u128 = modint(2, d, MOD) as u128;
    let pow_d_minus_2: u128 = if d>=2 { modint(2, d-2, MOD) as u128 } else { 0 };
    let mut ans: u128 = 0;
    if n*2 < d { println!("0");return; }
    for i in 0..n {
        // println!("ans: {}", ans);
        let h = n-i-1;
        if h*2 < d { break; }
        // iの高さの頂点から真っ直ぐ伸ばす
        if h >= d {
            ans += base * pow_d;
            ans %= MOD as u128;
        }
        // iの高さの頂点を挟んで伸ばす
        // println!("i: {}, d: {}, base: {}, d2: {}, n-i: {}", i, d, base, d2, n-i);
        if d >= 2 {
            let l = 1.max(d.saturating_sub(h));
            let u = h.min(d-1);
            if l <= u {
                let cnt = (u-l+1) as u128;
                ans += base*pow_d_minus_2*cnt;
                ans %= MOD as u128;
            }
        }
        base *= 2;
        base %= MOD as u128;
    }
    ans *= 2;
    ans %= MOD as u128;

    println!("{}", ans);
}