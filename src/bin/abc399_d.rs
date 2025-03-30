use proconio::input;
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            a: [usize; 2*n],
        }
        let mut set: HashSet<usize> = HashSet::new();
        for i in 0..(2*n-1) {
            if a[i] == a[i+1] {
                set.insert(a[i]);
            }
        }
        let mut map: HashMap<(usize, usize), usize> = HashMap::new();
        let mut pre = (0, 0);
        for i in 0..(2*n-1) {
            let a1 = a[i].min(a[i+1]);
            let a2 = a[i].max(a[i+1]);
            if pre == (a1, a2) { pre = (0, 0); continue; }
            pre = (a1, a2);
            if set.contains(&a1) || set.contains(&a2) { continue; }
            let e = map.entry((a1, a2)).or_insert(0);
            *e += 1;
        }
        let mut ans = 0;
        for (_, &cnt) in map.iter() {
            if cnt > 1 { ans += 1; }
        }
        // println!("map: {:?}", map);
        println!("{}", ans);
    }
}