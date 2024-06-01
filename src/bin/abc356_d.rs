use proconio::input;
use ac_library::modint;

fn main() {
    input! {
        n: i64,
        m: i64,
    }
    let r#mod = 998244353;
    let mut ni = Vec::new();
    let mut mi = Vec::new();
    let mut n2 = n;
    let mut m2 = m;
    while n2 > 0 {
        let i = n2 & (-n2);
        ni.push(i.ilog2());
        n2 -= i;
    }
    while m2 > 0 {
        let i = m2 & (-m2);
        mi.push(i.ilog2());
        m2 -= i;
    }
    let mut ans = 0;
    for i in mi.iter() {
        ans += ((((n+1) / (1i64<<(i+1)))%r#mod) * ((1i64<<i)%r#mod))%r#mod;
        ans %= r#mod;
        if ((n+1)%(1<<(i+1))) > 1<<i {
            ans += (((n+1) % (1i64<<(i+1))) - (1i64<<i))%r#mod;
            ans %= r#mod;
        }
    }
    println!("{}", ans);
}