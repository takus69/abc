use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        rc: [(usize, usize); m],
    }
    let mut col: HashMap<usize, usize> = HashMap::new();
    let mut row: HashMap<usize, usize> = HashMap::new();
    for &(r, c) in &rc {
        if col.contains_key(&c) {
            let rr = col.get(&c).unwrap();
            row.remove(rr);
        }
        if row.contains_key(&r) {
            let cc = row.get(&r).unwrap();
            col.remove(cc);
        }
        col.insert(c, r);
        row.insert(r, c);
    }
    println!("{}", col.len());
}