use proconio::input;

pub fn primes(n: usize) -> Vec<usize> {
    // エラトステネスの篩にて、n以下の素数リストを作成
    let mut is_prime: Vec<bool> = vec![true; n+1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=n {
        for j in ((i*2)..=n).step_by(i) {
            is_prime[j] = false;
        }
    }
    (0..=n).filter(|&i| is_prime[i]).collect()
}

fn main() {
    input! {
        n: usize,
    }
    // x^8 or x^2*y^2 の2種類
    // 素数を列挙
    let p = primes(((n as f64).sqrt() + 1.0) as usize);
    let mut ans = 0;
    for i in 0..p.len() {
        let pi = p[i];
        if pi * pi > n { break; }
        if pi <= ((n as f64).powf(1.0/8.0) as usize) && pi.pow(8) <= n {
            // println!("8: {}", pi.pow(8));
            ans += 1;
        }
        for j in (i+1)..p.len() {
            let pj = p[j];
            if pi*pj <= ((n as f64).sqrt() as usize) && pi*pi * pj*pj <= n {
                // println!("2-2: {}", pi*pi*pj*pj);
                ans += 1;
            } else { break; }
        }
    }
    println!("{}", ans);
}