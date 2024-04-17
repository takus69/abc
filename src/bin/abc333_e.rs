use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        tx: [(usize, usize); n],
    }
    let mut k = 0;
    let mut min_k = 0;
    let mut ans: Vec<usize> = Vec::new();
    let mut cnt: HashMap<usize, usize> = HashMap::new();

    for (t, x) in tx.iter().rev() {
        if t == &2 {
            let c = cnt.entry(*x).or_insert(0);
            *c += 1;
            k += 1;
            min_k = min_k.max(k);
        } else {
            let c = cnt.entry(*x).or_insert(0);
            if *c > 0 {
                *c -= 1;
                k -= 1;
                ans.push(1);
            } else {
                ans.push(0);
            }
        }
    }
    for (_, v) in cnt {
        if v > 0 {
            min_k = -1;
            break;
        }
    }

    println!("{}", min_k);
    if min_k >= 0 {
        println!("{}", ans.iter().rev().map(|x| x.to_string()).collect::<Vec<String>>().join(" "))
    }
    
}