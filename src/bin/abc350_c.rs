use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut v_i = HashMap::new();
    let mut i_v = HashMap::new();
    let mut ans = Vec::new();
    for (i, v) in a.iter().enumerate() {
        v_i.insert(*v, i+1);
        i_v.insert(i+1, *v);
    }
    for v in 1..=n {
        let i = v_i[&v];
        if v == i { continue; }
        let v2 = i_v[&v];
        v_i.insert(v, v);
        i_v.insert(v, v);
        v_i.insert(v2, i);
        i_v.insert(i, v2);
        ans.push((v, i));
    }

    println!("{}", ans.len());
    for (a1, a2) in ans {
        println!("{} {}", a1, a2);
    }
}