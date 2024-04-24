use proconio::input;

fn modint(x: usize, n: usize, r#mod: usize) -> usize {
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
        t: i64,
        nmk: [(i64, i64, i64); t],
    }
    for (n, m, k) in nmk {
        // println!("{} {} {}", n, m, k);
        // let mut n = n;
        if k == m-1 {  // k == m-1の場合は、2^m-2^kは2^(m-1)になるため、2^nが2^(m-1)以上ならmod2は0になる
            if n >= m-1 {
                println!("0");
            } else {
                println!("{}", modint(2, n as usize, 10));
            }
        } else {
            // mより小さくなるまでnからm-kを引く
            let mut v = n%(m-k);  // m-kで引けるだけ引く(mod m-k)
            v += (m-v-1)/(m-k)*(m-k);  // mより小さい(m-1)までm-kを足す。上記vから足すので、m-1-vをm-kで割った商の分だけm-kを足す
            if n < m { v = n; }  // mよりnが小さい場合は、m-1ではなくnに戻す必要がある
            println!("{}", modint(2, v as usize, 10));
        }
    }
}