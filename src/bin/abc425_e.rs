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
        t: usize,
        m: usize,
    }

    let mut comb: Vec<Vec<usize>> = vec![vec![0; 5001]; 5001];
    comb[0][0] = 1;
    for i in 1..=5000 {
        comb[i][0] = 1;
        for j in 1..=5000 {
            comb[i][j] = (comb[i-1][j-1] + comb[i-1][j]) % m;
        }
    }

    for _ in 0..t {
        input! {
            n:usize,
            c: [usize; n],
        }
        let mut ans = 1;
        let mut sum: usize = c.iter().sum();
        for &ci in c.iter() {
            ans *= comb[sum][ci];
            ans %= m;
            sum -= ci;
        }
        println!("{}", ans);
    }
}