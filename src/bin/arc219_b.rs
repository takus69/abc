use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            p: [usize; n],
        }
        let mut ans = 0;
        for i in 0..n {
            if p[i] == i+1 {
                ans += n-i-1;
                if i==n-1 { ans += 1; }
                ans %= MOD;
            } else {
                break;
            }
        }
        println!("{}", ans);
    }
}