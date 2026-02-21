use proconio::input;
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        m: usize,
        a: usize,
        b: usize,
    }

    let mut ans: HashSet<(usize, usize)> = HashSet::new();
    let mut not: HashSet<(usize, usize)> = HashSet::new();
    for x in 0..m {
        for y in 0..m {
            let (mut x, mut y) = (x, y);
            let mut tmp: HashSet<(usize, usize)> = HashSet::new();
            let mut flg = false;
            while !ans.contains(&(x, y)) && !tmp.contains(&(x, y)) && !not.contains(&(x, y)) {
                tmp.insert((x, y));
                let z = (a*y + b*x) % m;
                // println!("x: {}, y: {}, z: {}", x, y, z);
                if z == 0 || y == 0 || x == 0 {
                    flg = true;
                    break;
                }
                (y, x) = (z, y);
            }
            if !flg && !not.contains(&(x, y)) {
                ans.extend(tmp);
            } else {
                not.extend(tmp.clone());
            }
        }
    }
    // println!("ans: {:?}", ans);
    // println!("not: {:?}", not);

    println!("{}", ans.len());
}