use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
    }
    let mut k: Vec<usize> = Vec::new();
    let mut a: Vec<Vec<usize>>  = Vec::new();
    for _ in 0..n {
        input! {
            ki: usize,
            ai: [usize; ki],
        }
        k.push(ki);
        a.push(ai);
    }
    let mut a2: Vec<HashMap<usize, usize>> = Vec::new();
    for i in 0..n {
        let mut ai = HashMap::new();
        for j in 0..k[i] {
            let e = ai.entry(a[i][j]).or_insert(0);
            *e += 1;
        }
        a2.push(ai);
    }

    // 全組合せを試す
    let mut ans: f64 = 0.0;
    for i in 0..(n-1) {
        for j in (i+1)..n {
            let b1 = &a2[i];
            let b2 = &a2[j];
            let mut p = 0.0;
            for (key, &c1) in b1.iter() {
                let c2 = if let Some(&v) = b2.get(key) { v } else { 0 };
                p += ((c1 as f64) * (c2 as f64)) / ((k[i] as f64) * (k[j] as f64));
            }
            ans = ans.max(p);
        }
    }

    println!("{}", ans);
}