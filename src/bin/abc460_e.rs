use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    const MOD: usize = 998244353;
    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
        }

        fn gcd(a: usize, b: usize) -> usize {
            if b == 0 { return a; }
            gcd(b, a%b)
        }

        let mut ans = 0;
        for d in 1..=20 {  // dはyの桁数
            if 10usize.pow(d-1) > n { break; }
            let y_cnt = if 10usize.pow(d) <= n { 10usize.pow(d) } else { n+1 } - 10usize.pow(d-1);
            let g = gcd((10usize.pow(d)-1), m);
            let g2 = m/g;
            let x_cnt = if (10usize.pow(d)-1)%m == 0 {
                n
            } else {
                n/g2
            };
            // println!("d: {}, y_cnt: {}, x_cnt: {}", d, y_cnt, x_cnt);
            ans += (y_cnt%MOD) * (x_cnt%MOD);
            ans %= MOD;
        }
        println!("{}", ans);
    }
}