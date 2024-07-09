use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let r#mod = 998244353;
    let mut sum_a: usize = a.iter().sum();
    let mut ans = 0;
    for i in (1..n).rev() {
        let ai = a[i];
        sum_a -= ai;
        let k = ai.to_string().len();
        ans += (sum_a%r#mod) * ((10_i64.pow(k as u32) as usize)%r#mod);
        ans += (ai%r#mod)*i;
        ans %= r#mod;
    }
    println!("{}", ans);
}