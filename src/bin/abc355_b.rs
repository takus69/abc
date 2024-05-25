use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut c = Vec::new();
    for ai in a.iter() { c.push(ai); }
    for bi in b.iter() { c.push(bi); }
    c.sort();
    let mut ans = "No";
    let mut flg_a = false;
    for ci in c.iter() {
        if a.contains(&ci) {
            if flg_a {
                ans = "Yes";
                break;
            }
            flg_a = true;
        } else {
            flg_a = false;
        }

    }
    println!("{}", ans);
}