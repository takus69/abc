use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        l: usize,
        d: [usize; n-1],
    }
    if l%3 != 0 {
        println!("0");
        return;
    }
    let mut now = 0;
    let mut x: HashMap<usize, usize> = HashMap::new();
    x.insert(0, 1);
    for &di in d.iter() {
        now += di;
        now %= l;
        let e = x.entry(now).or_insert(0);
        *e += 1;
    }
    let mut ans = 0;
    for i in 0..(l/3) {
        let cnt1 = x.get(&i).unwrap_or(&0);
        let cnt2 = x.get(&(i+l/3)).unwrap_or(&0);
        let cnt3 = x.get(&(i+2*(l/3))).unwrap_or(&0);
        ans += cnt1*cnt2*cnt3;
    }
    println!("{}", ans);
}