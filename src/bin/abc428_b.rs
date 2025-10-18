use proconio::{input, marker::Chars};
use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }
    let mut map: HashMap<Vec<char>, usize> = HashMap::new();
    for i in 0..=(n-k) {
        let key = s[i..(i+k)].to_vec();
        if map.contains_key(&key) { continue; }
        let e = map.entry(key.clone()).or_insert(1);
        for j in (i+1)..=(n-k) {
            if s[j..(j+k)].to_vec() == key {
                *e += 1;
            }
        }
    }
    let mut ans_size = 0;
    let mut ans_vec: Vec<Vec<char>> = Vec::new();
    for (k, &v) in map.iter() {
        if ans_size < v {
            ans_size = v;
            ans_vec = Vec::new();
            ans_vec.push(k.clone());
        } else if ans_size == v {
            ans_vec.push(k.clone());
        }
    }
    ans_vec.sort();
    println!("{}", ans_size);
    for key in ans_vec.iter() {
        print!("{} ", key.iter().join(""))
    }
}