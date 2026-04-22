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
    nk: [(usize, usize); t],
  }
  for &(n, k) in nk.iter() {
    println!("{}", f(n, k));
  }
  
  fn comb(n: usize, k: usize) -> usize {
    let r#mod = 998244353;
    let mut ret = 1;
    for i in (n-k+1)..=n {
      ret *= i;
      ret %= r#mod;
    }
    for i in 1..=k {
      ret *= modinv(i, r#mod);
      ret %= r#mod;
    }
    
    ret
  }
  
  fn f(n: usize, k: usize) -> usize {
    let r#mod = 998244353;
    let n: Vec<char> = format!("{:b}", n).chars().collect();
    let m = n.len();
    let mut ans = 0;
    let mut cnt = 0;
    let mut sum = 0;
    for (i, &b) in n.iter().enumerate() {
      if b == '1' {
        let m2 = modint(2, m-i-1, r#mod);
        // println!("i: {}, b: {}, m2: {}, ans: {}, sum: {}, cnt: {}", i, b, m2, ans, sum, cnt);
        // 前からi桁目が'0'になる場合。残りの桁は好きな組み合わせが可能
        if m-i-1 >= k-cnt {  // i桁目が0のパターン
            // 前からi桁目より前
            let c = comb(m-i-1, k-cnt);
            ans += sum * c;
            ans %= r#mod;
            // 前からi桁目以降の好きな組み合わせ(1つの組合せあたり、m2-1 を m-i-1で割って、k-cnt倍した値が加算される)
            let c = comb(m-i-2, k-cnt-1);
            ans += (m2-1) * c;
            ans %= r#mod;
        }

        // 前からi桁目が'1'になる場合
        sum += m2;
        sum %= r#mod;
        cnt += 1;

        if cnt == k {
            ans += sum;  // 前から順に'1'がk個存在した場合
            ans %= r#mod;
            break;
        }
      }
    }

    ans
  }
}