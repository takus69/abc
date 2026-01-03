use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        q: usize,
    }
    let mut map: HashMap<String, usize> = HashMap::new();
    for _ in 0..q {
        input! {
            c: usize,
        }
        if c == 1 {
            input! {
                x: String,
                y: usize,
            }
            map.insert(x, y);
        } else {
            input! {
                x: String,
            }
            println!("{}", map.get(&x).unwrap());
        }
    }
}