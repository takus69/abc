use proconio::input;
use std::collections::HashMap;

struct Input {
    n: i64,
    a: i64,
    x: i64,
    y: i64,
}

fn main() {
    input! {
        n: i64,
        a: i64,
        x: i64,
        y: i64,
    }
    let input = Input{ n, a, x, y };
    let mut map = HashMap::new();
    map.insert(0, 0.0);
    let ans = f(n, &mut map, &input);
    println!("{}", ans);
}

fn f(n: i64, map: &mut HashMap<i64, f64>, input: &Input) -> f64 {
    if map.contains_key(&n) { return *map.get(&n).unwrap(); }
    let ret1 = input.x as f64 + f(n/input.a, map, input);
    let ret2 = (input.y as f64 * 6.0 / 5.0) + f(n/2, map, input) / 5.0 + f(n/3, map, input) / 5.0 + f(n/4, map, input) / 5.0 + f(n/5, map, input) / 5.0 + f(n/6, map, input) / 5.0;
    let ret = ret1.min(ret2);
    map.insert(n, ret);
    ret
} 