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
        n: usize,
        q: usize,
        s: Chars,
        abcd: [(usize, usize, usize, usize); q],
    }
    let mut hash: Vec<usize> = Vec::new();
    let mut b: usize = 1;
    let mut bs: Vec<usize> = vec![b];
    let mut r#mod = 1000000009;
    for &si in s.iter() {
        let n = si as usize - 'a' as usize + 1;
        hash.push((n*b)%r#mod);
        b *= 26;
        b %= r#mod;
        bs.push(b);
    }
    let mut hash2: Vec<usize> = vec![0];
    let mut b = 0;
    for &hi in hash.iter() {
        b += hi;
        b %= r#mod;
        hash2.push(b);
    }

    for &(a, b, c, d) in abcd.iter() {
        let h1 = ((r#mod + hash2[b] - hash2[a-1])*modinv(bs[a], r#mod))%r#mod;
        let h2 = ((r#mod + hash2[d] - hash2[c-1])*modinv(bs[c], r#mod))%r#mod;
        if h1 != h2 {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}