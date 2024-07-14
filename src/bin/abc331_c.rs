use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut sum_a: usize = 0;
    let mut a2: Vec<(usize, usize)> = Vec::new();
    let mut map: HashMap<usize, usize> = HashMap::new();
    for (i, ai) in a.iter().enumerate() {
        sum_a += ai;
        a2.push((*ai, i));
    }

    a2.sort();
    for (ai, _) in a2.iter() {
        sum_a -= ai;
        map.insert(*ai, sum_a);
    }
    let mut ans: Vec<usize> = vec![0; n];
    for (ai, i) in a2.iter() {
        ans[*i] = map[ai];
    }
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}