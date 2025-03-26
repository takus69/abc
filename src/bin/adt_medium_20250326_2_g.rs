use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }
    a.sort();
    let mut i = a[0];
    let mut sum: HashMap<usize, usize> = HashMap::new();
    let mut pre_ai = a[0];
    for &ai in a.iter() {
        if pre_ai == ai || pre_ai+1 == ai {
            let e = sum.entry(i).or_insert(0);
            *e += ai;
        } else {
            i = ai;
            let e = sum.entry(i).or_insert(0);
            *e += ai;
        }
        pre_ai = ai;
    }
    if a[a.len()-1] == m-1 && i != 0 && sum.contains_key(&0) {
        let &v = sum.get(&0).unwrap();
        let e = sum.entry(i).or_insert(0);
        *e += v;
    }
    let sum_a: usize = a.iter().sum();
    let mut ans = usize::MAX;
    for (_, &s) in sum.iter() {
        ans = ans.min(sum_a - s);
    }
    println!("{}", ans);
}