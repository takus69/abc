use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        ab: [(usize, usize); n],
    }
    let mut ab2: Vec<(usize, usize, usize)> = Vec::new();
    for (i, &(a, b)) in ab.iter().enumerate() {
        ab2.push((a, b, i));
    }
    
    let mut ans: HashMap<usize, (usize, usize)> = HashMap::new();

    // h
    ab2.sort_by(|a, b| a.0.cmp(&b.0));
    let mut pre = ab2[0].0;
    let mut hi = 1;
    for &(a, _, i) in ab2.iter() {
        if pre == a {
            ans.insert(i, (hi, usize::MAX));
        } else {
            hi += 1;
            ans.insert(i, (hi, usize::MAX));
        }
        pre = a;
    }

    // w
    ab2.sort_by(|a, b| a.1.cmp(&b.1));
    let mut pre = ab2[0].1;
    let mut wi = 1;
    for &(_, b, i) in ab2.iter() {
        let &(hi, _) = ans.get(&i).unwrap();
        if pre == b {
            ans.insert(i, (hi, wi));
        } else {
            wi += 1;
            ans.insert(i, (hi, wi));
        }
        pre = b;
    }

    for i in 0..n {
        let &(a, b) = ans.get(&i).unwrap();
        println!("{} {}", a, b)
    }
}