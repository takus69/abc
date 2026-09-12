use proconio::{input, marker::Chars};
use std::collections::HashMap;

pub fn primes(n: usize) -> Vec<usize> {
    // エラトステネスの篩にて、n以下の素数リストを作成
    // 計算量O(N log log N)
    let mut is_prime: Vec<bool> = vec![true; n+1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2.. {
        if i*i > n { break; }
        for j in ((i*i)..=n).step_by(i) {
            is_prime[j] = false;
        }
    }
    (0..=n).filter(|&i| is_prime[i]).collect()
}

fn main() {
    input! {
        s: Chars,
    }
    let p = primes(10_000_000);
    let mut map: HashMap<char, Vec<usize>> = HashMap::new();
    for (i, &si) in s.iter().enumerate() {
        let e = map.entry(si).or_insert(Vec::new());
        e.push(i);
    }
    let base = 10usize.pow(s.len() as u32);
    for &pi in &p {
        let ans_pi = pi;
        if pi >= base || pi < base/10 { continue; }
        let mut digit: Vec<usize> = Vec::new();
        let mut pi = pi;
        while pi > 0 {
            digit.push(pi%10);
            pi /= 10;
        }
        digit.reverse();
        let mut flg = true;
        let mut vs: Vec<Vec<usize>> = Vec::new();
        for (_, v) in map.iter() {
            let c = digit[v[0]];
            for &vi in v {
                if c != digit[vi] {
                    flg = false;
                    break;
                }
            }
            if !flg { break; }
            vs.push(v.clone());
        }
        if !flg { continue; }
        for i in 0..(vs.len()-1) {
            for j in (i+1)..vs.len() {
                if digit[vs[i][0]] == digit[vs[j][0]] {
                    flg = false;
                    break;
                }
            }
            if !flg { break; }
        }
        if flg {
            println!("{}", ans_pi);
            return;
        }
    }
    println!("-1");
}