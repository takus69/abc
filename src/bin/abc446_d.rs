use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map: HashMap<usize, usize> = HashMap::new();
    for &ai in a.iter() {
        map.insert(ai, 1);
        let flg = map.contains_key(&(ai-1));
        if flg {
            let c = *(map.get(&(ai-1)).unwrap());
            let e = map.entry(ai).or_insert(1);
            if c+1 > *e {
                *e = c+1;
            }
        }
    }
    // println!("map: {:?}", map);
    
    let mut ans = 1;
    for (_, &c) in map.iter() {
        ans = ans.max(c);
    }

    println!("{}", ans);
}