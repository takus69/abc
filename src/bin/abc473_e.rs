use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut sum = 0;
    let mut sum_a: Vec<usize> = vec![sum];
    for &ai in &a {
        sum += ai;
        sum %= k;
        sum_a.push(sum)
    }

    let mut set: HashSet<usize> = HashSet::new();
    let mut ans = 0;
    for &v in &sum_a {
        if set.contains(&v) {
            ans += 1;
            set = HashSet::new();
            set.insert(v);
        }
        set.insert(v);
    }

    println!("{}", ans);
}