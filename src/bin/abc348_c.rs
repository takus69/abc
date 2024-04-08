use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        ac: [(usize, usize); n],
    }

    let mut map: HashMap<usize, usize> = HashMap::new();
    let inf = std::usize::MAX;
    for (a, c) in ac {
        let tmp = map.get(&c).unwrap_or(&inf).min(&a);
        map.insert(c, *tmp);
    }
    let mut ans = 0;
    for (_, v) in map {
        ans = ans.max(v);
    }

    println!("{}", ans);
}
