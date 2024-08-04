use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [String; n],
        d: [String; m],
        p: [usize; m+1],
    }
    let mut price: HashMap<String, usize> = HashMap::new();
    let other = p[0];
    for i in 0..m {
        price.insert(d[i].clone(), p[i+1]);
    }
    let mut ans = 0;
    for ci in c.iter() {
        ans += match price.get(ci) {
            Some(v) => { *v },
            None => { other },
        }
    }
    println!("{}", ans);
}