use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }
    let mut map: Vec<HashMap<usize, usize>> = vec![HashMap::new(); 11];
    for &ai in a.iter() {
        for j in 0..11 {
            let key = (ai * 10_usize.pow(j)) % m;
            let e = map[j as usize].entry(key).or_insert(0);
            *e += 1;
        }
    }
    let mut ans = 0;
    for &ai in a.iter() {
        let l = ai.to_string().len();
        let key = (m-ai%m)%m;
        ans += map[l].get(&key).unwrap_or(&0);
    }
    println!("{}", ans);
}