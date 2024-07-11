use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut start = 0;
    for (i, ai) in a.iter().enumerate() {
        if ai < &0 {
            start = i+1;
        } else {
            map.insert(*ai as usize, i+1);
        }
    }
    let mut ans: Vec<usize> = Vec::new();
    ans.push(start);
    for _ in 0..(n-1) {
        start = map[&start];
        ans.push(start);
    }
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}