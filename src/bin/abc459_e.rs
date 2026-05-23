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

fn main() {
    input! {
        n: usize,
        p: [usize; n-1],
        c: [usize; n],
        d: [usize; n],
    }
    const MOD: usize = 998244353;
    let mut edge: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut p2: Vec<usize> = vec![0; n];
    for (i, &pi) in p.iter().enumerate() {
        let child = i+1;
        let parent = pi-1;
        p2[child] = parent;
        edge[parent].push(child);
    }
    let p = p2;


    fn dfs(i: usize, p: &[usize], c: &[usize], d: &[usize], edge: &[Vec<usize>]) -> (usize, usize) {
        let mut ret = 1;
        let mut left_sum = 0;
        for &child in edge[i].iter() {
            let (cnt, left) = dfs(child, p, c, d, edge);
            ret *= cnt;
            ret %= MOD;
            left_sum += left;
        }

        left_sum += c[i];
        // left_sumからd[i]を取る組合せ
        if left_sum < d[i] {
            return (0, 0);
        }
        for j in 0..d[i] {
            ret *= (left_sum-j)%MOD;  // 階乗
            ret %= MOD;
            ret *= modinv(d[i]-j, MOD);  // 階乗の逆元
            ret %= MOD;
        }
        left_sum -= d[i];

        (ret, left_sum)
    }

    let (ans, _) = dfs(0, &p, &c, &d, &edge);
    println!("{}", ans);
}