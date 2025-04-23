use proconio::input;
use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    }
    let mut q2i: HashMap<usize, usize> = HashMap::new();
    let mut i2q: HashMap<usize, usize> = HashMap::new();
    for (i, &qi) in q.iter().enumerate() {
        q2i.insert(qi, i+1);
        i2q.insert(i+1, qi);
    }
    let mut i2p: HashMap<usize, usize> = HashMap::new();
    for (i, &pi) in p.iter().enumerate() {
        i2p.insert(i+1, pi);
    }
    let mut ans: Vec<usize> = Vec::new();
    for i in 1..=n {
        ans.push(*i2q.get(&i2p.get(q2i.get(&i).unwrap()).unwrap()).unwrap());
    }

    println!("{}", ans.iter().join(" "));
}