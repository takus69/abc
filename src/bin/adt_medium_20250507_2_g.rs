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
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }
    let ps = primes(b+d);
    for i in a..=b {
        let mut tmp = false;
        for j in c..=d {
            if ps.contains(&(i+j)) {
                tmp = true;
            }
        }
        if tmp {
            continue;
        } else {
            println!("Takahashi");
            return;
        }
    }
    println!("Aoki");
}