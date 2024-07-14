use proconio::input;
use std::collections::HashMap;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }
    let mut candidate: usize = 0;
    let mut max_vote = 0;
    let mut votings: HashMap<usize, usize> = HashMap::new();
    for ai in a.iter() {
        let e = votings.entry(*ai).or_insert(0);
        *e += 1;
        match &(*e).cmp(&max_vote) {
            Ordering::Greater => {
                max_vote = *e;
                candidate = *ai;
            },
            Ordering::Equal => {
                candidate = candidate.min(*ai);
            },
            _ => {},
        }
        println!("{}", candidate);
    }
}