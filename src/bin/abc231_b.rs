use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut cnt: HashMap<String, usize> = HashMap::new();
    for si in &s {
        let e = cnt.entry(si.clone()).or_insert(0);
        *e += 1;
    }
    let mut ans = "";
    let mut max_cnt = 0;
    for (s, &c) in &cnt {
        if max_cnt < c {
            ans = s;
            max_cnt = c;
        }
    }
    println!("{}", ans);
}