use proconio::input;
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        mut rca: [(usize, usize, usize); n],
    }
    let mut map: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    let mut set: HashSet<usize> = HashSet::new();
    for i in 0..n {
        rca[i].0 -= 1;
        rca[i].1 -= 1;
        let (r, c, a) = rca[i];
        let e = map.entry(a).or_insert(Vec::new());
        e.push((r, c));
        set.insert(a);
    }
    let mut dp: HashMap<(usize, usize), usize> = HashMap::new();
    let mut rmax: Vec<usize> = vec![0; h];
    let mut cmax: Vec<usize> = vec![0; w];
    let mut order: Vec<usize> = set.into_iter().collect();
    order.sort();order.reverse();
    for &a in &order {
        for &(r, c) in map.get(&a).unwrap().iter() {
            let e = dp.entry((r, c)).or_insert(0);
            *e = (*e).max(rmax[r]).max(cmax[c]);
        }
        for &(r, c) in map.get(&a).unwrap().iter() {
            let e = dp.entry((r, c)).or_insert(0);
            rmax[r] = rmax[r].max(*e+1);
            cmax[c] = cmax[c].max(*e+1);
        }
    }
    for &(r, c, _) in &rca {
        println!("{}", dp.get(&(r, c)).unwrap());
    }
}