use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut map: HashMap<String, usize> = HashMap::new();
    for si in s.iter() {
        let &cnt = map.get(si).unwrap_or(&0);
        if cnt == 0 {
            println!("{}", si);
        } else {
            println!("{}({})", si, cnt);
        }
        let e = map.entry(si.clone()).or_insert(0);
        *e += 1;
    }
}