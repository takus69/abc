use proconio::input;
use std::collections::{HashSet, HashMap};

fn main() {
    input! {
        n: usize,
        m: usize,
        adb: [(usize, usize, usize); n],
    }
    let mut dab: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();  // D日目に、A=>Bになる
    let mut cnt: HashMap<usize, usize> = HashMap::new();  // 現時点の種類の鳥の数
    let mut ans: Vec<usize> = Vec::new();
    for i in 0..n {
        let (a, d, b) = adb[i];
        let e = dab.entry(d).or_insert(Vec::new());
        e.push((a, b));
        let c = if d==1 { b } else { a };
        let e = cnt.entry(c).or_insert(0);
        *e += 1;
    }
    ans.push(cnt.len());
    for d in 2..=m {
        if !dab.contains_key(&d) { ans.push(cnt.len());continue; }
        for &(a, b) in dab.get(&d).unwrap().iter() {
            let e = cnt.entry(a).or_insert(0);
            *e -= 1;
            if *e == 0 {
                cnt.remove(&a);
            }
            let e = cnt.entry(b).or_insert(0);
            *e += 1;
        }
        ans.push(cnt.len());
    }

    for a in &ans {
        println!("{}", a);
    }
}