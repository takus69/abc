use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }
    let mut sum_a = 0;
    let mut sum_a_m: Vec<usize> = Vec::new();
    for i in 0..(2*n) {
        sum_a += a[i%n];
        sum_a %= m;
        sum_a_m.push(sum_a);
    }
    let mut map: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        let a = sum_a_m[i];
        let e = map.entry(a).or_insert(0);
        *e += 1;
    }
    let mut ans = 0;
    // println!("map: {:?}, sum_a_m: {:?}", map, sum_a_m);
    for i in 0..n {
        let ai = sum_a_m[i];
        ans += map.get(&ai).unwrap()-1;
        let e = map.get_mut(&ai).unwrap();
        *e -= 1;
        let ai = sum_a_m[i+n];
        let e = map.entry(ai).or_insert(0);
        *e += 1;
        // println!("map: {:?}", map);
    }
    println!("{}", ans);
}
