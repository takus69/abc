use proconio::{input, marker::Chars};
use std::collections::HashMap;

fn main() {
    input! {
        x: Chars,
        n: usize,
        s: [Chars; n],
    }
    let mut map: HashMap<char, char> = HashMap::new();
    let abc: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    for (i, &xi) in x.iter().enumerate() {
        map.insert(xi, abc[i]); 
    }
    let mut s_conv: Vec<(Vec<char>, usize)> = Vec::new();
    for (i, si) in s.iter().enumerate() {
        let mut tmp: Vec<char> = Vec::new();
        for c in si {
            tmp.push(map[c]);
        }
        s_conv.push((tmp, i));
    }
    s_conv.sort();
    for (_, i) in &s_conv {
        println!("{}", s[*i].iter().cloned().collect::<String>());
    }

}