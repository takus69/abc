use proconio::input;
use std::collections::HashSet;

fn solve(wv: &Vec<(i32, i32)>, x: &Vec<i32>) -> i32 {
    // println!("{:?} {:?}", wv, x);
    let mut x = x.clone();
    x.sort();
    let mut used = HashSet::new();
    let mut ans = 0;
    for xi in &x {
        let mut max_i = 100;
        let mut max_v = 0;
        for (i, (w, v)) in wv.iter().enumerate() {
            if xi < w || used.contains(&i) { continue; }
            if &max_v < v {
                max_i = i;
                max_v = *v;
            }
        }
        ans += max_v;
        if max_i != 100 {
            used.insert(max_i);
        }

    }
    ans
}

fn main() {
    input! {
        n: i32,
        m: i32,
        q: i32,
        wv: [(i32, i32); n],
        mut x: [i32; m],
    }
    // x.push(0);
    // println!("{:?}", x);

    for _ in 0..q {
        input! {
            mut l: usize,
            mut r: usize,
        }
        l -= 1;
        r -= 1;
        let ans = solve(&wv, &[&x[0..l], &x[r+1..]].concat());
        println!("{}", ans);
    }
}