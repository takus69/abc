use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut map: HashMap<usize, (i64, i64)> = HashMap::new();
    for (l, r) in lr {
        // 開始l
        let entry = map.entry(l).or_insert((0, 0));
        (*entry).0 += 1;

        // 終了r+1
        let entry = map.entry(r+1).or_insert((0, 0));
        (*entry).1 -= 1;
    }
    
    let mut kv = Vec::new();
    for (k, v) in map.iter() {
        kv.push((*k, *v));
    }
    kv.sort();
    // println!("{:?}", kv);

    let mut ans = 0;
    let mut pre = 0;
    for (k, v) in kv.iter() {
        let next = v.1 + v.0 + pre;
        if v.0 > 0 {
            ans += (pre+v.1) * v.0;
            ans += (v.0 * (v.0-1)) / 2;
        }
        pre = next;
    }
    println!("{}", ans);
}