use proconio::input;
use ac_library::Dsu;

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
        m: usize,
        uv: [(usize, usize); m],
    }

    let r#mod = 998244353;
    let mut dsu: Dsu = Dsu::new(n+1);
    let mut ans = 0;
    let mut cnt = n;
    for (i, &(u, v)) in uv.iter().enumerate().rev() {
        if !dsu.same(u, v) && cnt > 2 {
            dsu.merge(u, v);
            cnt -= 1;
        } else if !dsu.same(u, v) {
            ans += modint(2, i+1, r#mod);
            ans %= r#mod;
        }
    }

    println!("{}", ans);
}