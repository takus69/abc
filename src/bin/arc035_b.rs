use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: i64,
        t: [i64; n],
    }

    let mut cnt = HashMap::new();
    for ti in t {
        let c = cnt.entry(ti).or_insert(0);
        *c += 1;
    }

    let mut cnt: Vec<(&i64, &i64)> = cnt.iter().collect();
    cnt.sort_by(|a, b| a.0.cmp(&b.0));

    const MOD: i64 = 1_000_000_007;
    let mut score = 0;
    let mut ans = 1;
    let mut pre = 0;


    for (k, v) in cnt {
        // println!("score: {}, pre: {}, k: {}, v: {}", score, pre, k, v);
        score += k*v*(v+1)/2 + pre*v;
        pre += k*v;
        for i in 1..(v+1) {
            ans *= i;
            ans %= MOD;
        }
    }

    println!("{}", score);
    println!("{}", ans);
}