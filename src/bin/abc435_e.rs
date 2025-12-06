use proconio::input;
use ac_library::FenwickTree;
use std::collections::{HashSet, HashMap};

fn main() {
    input! {
        n: isize,
        q: isize,
        lr: [(usize, usize); q],
    }
    let mut map: HashSet<isize> = HashSet::new();
    for &(l, r) in lr.iter() {
        map.insert(l as isize);
        map.insert(r as isize);
    }
    map.insert(n);
    map.insert(0);
    let n2 = map.len();
    let mut order: Vec<isize> = map.into_iter().collect();
    order.sort();

    let mut fw: FenwickTree<isize> = FenwickTree::new(n2, 0);
    let mut pre = 0;
    let mut index: HashMap<isize, usize> = HashMap::new();
    let mut visited: HashSet<usize> = HashSet::new();
    for (i, &v) in order.iter().enumerate() {
        fw.add(i, v-pre);
        pre = v;
        index.insert(v, i);
    }
    for &(l, r) in lr.iter() {
        let li = *index.get(&(l as isize)).unwrap();
        let ri = *index.get(&(r as isize)).unwrap();
        let mut s = if li < ri {
            fw.sum((li+1)..=ri)
        } else { 0 };
        for j in 0..(ri-li) {
            let ri2 = ri-j;
            let rs = fw.sum(ri2..=ri2);
            visited.insert(order[ri2] as usize);
            if s > rs {
                fw.add(ri2, -rs);
                s -= rs;
            } else {
                fw.add(ri2, -s);
                break;
            }

            let li2 = li+j+1;
            let ls = fw.sum(li2..=li2);
            visited.insert(order[li2] as usize);
            if s > ls {
                fw.add(li2, -ls);
                s -= ls;
            } else {
                fw.add(li2, -s);
                break;
            }
        }
        if !visited.contains(&l) {
            fw.add(li, -1);
        }
        visited.insert(l);
        visited.insert(r);

        println!("{}", fw.sum(0..n2));
    }
}