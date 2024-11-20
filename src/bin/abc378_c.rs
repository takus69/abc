use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut map: HashMap<usize, i64> = HashMap::new();
    for ai in a.iter() {
        map.insert(*ai, -1);
    }
    let mut ans: Vec<i64> = Vec::new();
    for i in 0..n {
        let ai = a[i];
        let v = map.get(&ai).unwrap();
        ans.push(*v);
        map.insert(ai, (i+1) as i64);
    }
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}