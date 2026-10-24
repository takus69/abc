use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        q: usize,
    }
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut max_id = 0;
    for _ in 0..q {
        input! {
            s: String,
        }
        let id = if map.contains_key(&s) {
            let id = map.get(&s).unwrap();
            *id
        } else {
            max_id += 1;
            map.insert(s.clone(), max_id);
            max_id
        };
        println!("{}", id);
    }
}