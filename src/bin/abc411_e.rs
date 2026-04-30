use proconio::input;
use std::collections::{HashSet, HashMap};

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

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        a: [[usize; 6]; n],
    }

    let mut map: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (i, ai) in a.iter().enumerate() {
        for &ai2 in ai {
            let e = map.entry(ai2).or_insert(HashSet::new());
            e.insert(i);
        }
    }
    let mut s: Vec<usize> = map.keys().cloned().into_iter().collect();
    s.sort();
    s.reverse();

    let mut b: Vec<usize> = vec![0; n];
    let mut cnt = 0;
    let mut visited: Vec<bool> = vec![false; n];
    let mut pre_si = 0;
    // すべてのサイコロの最小値の最大値を探索
    while cnt < n {
        let si = s.pop().unwrap();
        for &i in map.get(&si).unwrap().iter() {
            let c = a[i].iter().filter(|&&x| x==si).count();
            b[i] += c;
            b[i] %= MOD;
            if !visited[i] {
                visited[i] = true;
                cnt += 1;
            }
        }
        pre_si = si;
    }
    let mut base = 1;
    for &bi in &b {
        base *= bi;
        base %= MOD;
        base *= modinv(6, MOD);
        base %= MOD;
    }
    // println!("base: {}, pre_si: {}", base, pre_si);
    // println!("b: {:?}", b);

    // 順に引いていく
    if s.is_empty() {
        println!("{}", pre_si%MOD);
        return;
    }
    let mut ans = s[0]%MOD;
    // println!("ans: {}", ans);
    while let Some(si) = s.pop() {
        ans += MOD - ((si-pre_si)*base)%MOD;
        ans %= MOD;
        // println!("ans: {}", ans);

        // baseを更新
        for &i in map.get(&si).unwrap().iter() {
            let c = a[i].iter().filter(|&&x| x==si).count();
            let pre_bi = b[i];
            b[i] += c;
            b[i] %= MOD;
            base *= modinv(pre_bi, MOD);
            base %= MOD;
            base *= b[i];
            base %= MOD;
        }

        pre_si = si;
    }
    
    println!("{}", ans);
}