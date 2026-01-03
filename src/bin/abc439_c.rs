use proconio::input;
use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
    }
    let mut cands: Vec<usize> = Vec::new();
    for x in 1..4000 {
        if x*x > n { break; }
        cands.push(x*x);
    }
    let mut map: HashMap<usize, usize> = HashMap::new();
    for i in 0..(cands.len()-1) {
        let x = cands[i];
        for j in (i+1)..cands.len() {
            let y = cands[j];
            if x+y > n { break; }
            let e = map.entry(x+y).or_insert(0);
            *e += 1;
        }
    }
    let mut ans: Vec<usize> = Vec::new();
    for (&a, &c) in map.iter() {
        if c == 1 {
            ans.push(a);
        }
    }
    ans.sort();
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}