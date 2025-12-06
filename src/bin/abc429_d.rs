use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: usize,
        a: [usize; n],
    }
    let mut cnt: HashMap<usize, usize> = HashMap::new();
    for &ai in a.iter() {
        let e = cnt.entry(ai).or_insert(0);
        *e += 1;
    }
    let mut keys: Vec<usize> = Vec::new();
    for (&k, _) in cnt.iter() {
        keys.push(k);
    }
    keys.sort();

    let mut ans = 0;
    let mut t = keys[0];
    let mut now = 0;
    let mut pre_s = *keys.last().unwrap();
    let mut j = 0;
    for (i, &s) in keys.iter().enumerate() {
        while now < c {
            t = keys[j];
            now += cnt.get(&t).unwrap();
            j += 1;
            j %= keys.len();
        }

        ans += now*((s+m-1-pre_s)%m+1);
        now -= cnt.get(&s).unwrap();
        pre_s = s;
    }

    println!("{}", ans);
}