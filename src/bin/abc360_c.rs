use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        w: [usize; n],
    }
    let mut cnt = HashMap::new();
    let mut cost = HashMap::new();
    for i in 0..n {
        let ai = a[i];
        let c1 = cnt.entry(ai).or_insert(0);
        *c1 += 1;
        let c2 = cost.entry(ai).or_insert(vec![]);
        let wi = w[i];
        c2.push(wi);
    }
    let mut ans = 0;
    for ai in cnt.keys() {
        if cnt.get(ai).unwrap() > &1 {
            let mut c2 = cost.get(ai).unwrap().clone();
            c2.sort();
            for i in 0..(c2.len()-1) {
                ans += c2[i];
            }
        }
    }
    // println!("cnt: {:?}, cost: {:?}", cnt, cost);
    println!("{}", ans);
}