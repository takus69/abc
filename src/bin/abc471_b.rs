use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut map: HashMap<String, usize> = HashMap::new();
    for si in &s {
        let si = si.to_ascii_uppercase();
        let e = map.entry(si.clone()).or_insert(0);
        *e += 1;
    }
    let mut ans = 0;
    for (si, &cnt) in map.iter() {
        ans = ans.max(cnt);
    }
    println!("{}", ans);
}