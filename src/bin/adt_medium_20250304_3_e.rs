use proconio::input;
use std::collections::{HashSet, HashMap};

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut b: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    let mut c: HashMap<usize, HashSet<usize>> = HashMap::new();
    for _ in 0..q {
        input! {
            a: usize,
        }
        if a == 1 {
            input! {
                i: usize,
                j: usize,
            }
            b[j].push(i);
            let e = c.entry(i).or_insert(HashSet::new());
            e.insert(j);
        } else {
            input! {
                i: usize,
            }
            if a == 2 {
                let mut bi = b[i].clone();
                bi.sort();
                println!("{}", bi.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
            } else {
                let mut ci: Vec<&usize> = c.get(&i).unwrap().iter().collect();
                ci.sort();
                println!("{}", ci.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
            }
        }
    }
}