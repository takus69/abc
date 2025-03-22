use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        a: [usize; 7],
    }
    let mut map: HashMap<usize, usize> = HashMap::new();
    for &ai in a.iter() {
        let e = map.entry(ai).or_insert(0);
        *e += 1;
    }
    let mut flg3 = false;
    let mut flg2 = false;
    for (ai, cnt) in map.iter() {
        if !flg3 && cnt >= &3 {
            flg3 = true;
        } else if cnt >= &2 {
            flg2 = true;
        }
    }
    if flg3 && flg2 {
        println!("Yes");
    } else {
        println!("No");
    }
}