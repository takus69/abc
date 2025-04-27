use proconio::input;
use std::collections::{HashSet, HashMap};

fn main() {
    input! {
        n: usize,
        d: usize,
        mut a: [usize; n],
    }
    a.sort();
    let mut map: HashMap<usize, usize> = HashMap::new();
    for &ai in a.iter() {
        let e = map.entry(ai).or_insert(0);
        *e += 1;
    }
    if d == 0 {
        let mut ans = 0;
        for (_, &cnt) in map.iter() {
            ans += cnt-1;
        }
        println!("{}", ans);
        return;
    }
    let mut visited: HashSet<usize> = HashSet::new();
    let mut ans = 0;
    for i in 0..n {
        let mut ai = a[i];
        if visited.contains(&ai) { continue; }
        let mut pre= (0, 0);  // 0: 削除しない、1: 削除
        let mut dp = (0, 0);
        loop {
            let &cnt = map.get(&ai).unwrap();
            dp.0 = pre.1;
            dp.1 = (pre.0+cnt).min(pre.1+cnt);
            visited.insert(ai);
            ai += d;
            if !map.contains_key(&ai) { break; }
            pre.0 = dp.0;
            pre.1 = dp.1;
        }
        ans += dp.0.min(dp.1);
    }
    println!("{}", ans);
}