use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut map: HashMap<usize, usize> = HashMap::new();
    for &ai in a.iter() {
        let e = map.entry(ai).or_insert(0);
        *e += 1;
    }
    let sum_a: usize = a.iter().sum();
    let mut ans = 0;
    for (_, &cnt) in map.iter() {
        if cnt < 2 { continue; }
        ans += (cnt*(cnt-1)/2) * (n-cnt);
    }
    println!("{}", ans);
}