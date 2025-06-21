use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; q],
    }
    let mut map: HashMap<usize, bool> = HashMap::new();
    let mut ans: usize = 0;
    for &ai in a.iter() {
        if map.contains_key(&ai) {
            map.remove(&ai);
            if map.contains_key(&(ai+1)) && map.contains_key(&(ai-1)) {
                ans += 1;
            } else if !map.contains_key(&(ai+1)) && !map.contains_key(&(ai-1)) {
                ans -= 1;
            }
        } else {
            map.insert(ai, true);
            if map.contains_key(&(ai+1)) && map.contains_key(&(ai-1)) {
                ans -= 1;
            } else if !map.contains_key(&(ai+1)) && !map.contains_key(&(ai-1)) {
                ans += 1;
            }
        }
        // println!("map: {:?}", map);
        println!("{}", ans);
    }

}