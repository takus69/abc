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
    let mut max_a = 0;
    for (ai, cnt) in map.iter() {
        if cnt == &1 {
            max_a = max_a.max(*ai);
        }
    }
    let mut ans = -1;
    for (i, &ai) in a.iter().enumerate() {
        if ai == max_a {
            ans = (i+1) as isize;
        }
    }
    println!("{}", ans);
}