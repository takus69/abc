use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        q: usize,
    }
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut ans = 0;
    for _ in 0..q {
        input! {
            qi: usize,
        }
        if qi == 1 || qi == 2 {
            input! {
                x: usize,
            }
            if qi == 1 {
                let e = map.entry(x).or_insert(0);
                *e += 1;
                if e == &1 {
                    ans += 1;
                }
            } else {
                let e = map.entry(x).or_insert(0);
                *e -= 1;
                if e == &0 {
                    ans -= 1;
                }
            }
        } else {
            println!("{}", ans);
        }
    }
}