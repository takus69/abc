use proconio::input;
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        xr: [(isize, isize); n],
    }
    
    let mut pos_cnt: HashMap<isize, usize> = HashMap::new();
    let mut pos_i: HashMap<isize, HashSet<usize>> = HashMap::new();
    for (i, &(x, r)) in xr.iter().enumerate() {
        let e = pos_cnt.entry(x+r).or_insert(0);
        *e += 1;
        let e = pos_i.entry(x+r).or_insert(HashSet::new());
        e.insert(i);
        let e = pos_cnt.entry(x-r).or_insert(0);
        *e += 1;
        let e = pos_i.entry(x-r).or_insert(HashSet::new());
        e.insert(i);
    }

    fn check(p: isize, pos_cnt: &mut HashMap<isize, usize>, pos_i: &mut HashMap<isize, HashSet<usize>>, xr: &Vec<(isize, isize)>) -> usize {
        // println!("check {}, pos_cnt: {:?}, pos_i: {:?}", p, pos_cnt, pos_i);
        let i = *(pos_i.get(&p).unwrap().iter().next().unwrap());
        let (x, r) = xr[i];
        let p2 = if p == (x+r) { x-r } else { x+r };
        pos_cnt.remove(&p);
        pos_i.remove(&p);

        let e = pos_cnt.entry(p2).or_insert(0);
        *e -= 1;
        let e2 = pos_i.entry(p2).or_insert(HashSet::new());
        // println!("check2: {}, e: {}", p2, e);
        e2.remove(&i);
        if e == &0 {
            // println!("check3, 1: {}, e: {}", p2, e);
            pos_cnt.remove(&p2);
            pos_i.remove(&p2);
            1
        } else if e == &1 {
            // println!("check3, 2: {}, e: {}", p2, e);
            check(p2, pos_cnt, pos_i, xr)+1
        } else {
            // println!("check3, 3: {}, e: {}", p2, e);
            1
        }
    }

    let mut ans: usize = 0;
    let pos_cnt2 = pos_cnt.clone().into_iter().collect::<Vec<(isize, usize)>>();
    for &(p, c) in pos_cnt2.iter() {
        if pos_cnt.contains_key(&p) && pos_cnt.get(&p) == Some(&1) {
            ans += check(p, &mut pos_cnt, &mut pos_i, &xr);
        }
        // println!("p: {}, ans: {}", p, ans);
    }
    ans += pos_cnt.len();
    println!("{}", ans);
}