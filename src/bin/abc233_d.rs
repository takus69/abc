use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: isize,
        a: [isize; n],
    }
    let mut cnt: HashMap<isize, usize> = HashMap::new();
    let mut sum = 0;
    for &ai in &a {
        sum += ai;
        let e = cnt.entry(sum).or_insert(0);
        *e += 1;
    }

    let mut ans = *cnt.get(&k).unwrap_or(&0);
    let mut sum = 0;
    for &ai in &a {
        sum += ai;
        let e = cnt.get_mut(&sum).unwrap();
        *e -= 1;
        ans += *cnt.get(&(sum+k)).unwrap_or(&0);
    }
    println!("{}", ans);
}