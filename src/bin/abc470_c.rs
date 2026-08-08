use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut val: Vec<usize> = vec![0; n+1];
    let mut ans = 0;
    let mut target: HashSet<usize> = HashSet::new();
    for _ in 0..q {
        input! {
            c: usize,
        }
        if c == 1 {
            input! {
                x: usize,
            }
            ans ^= val[x];
            val[x] += 1;
            ans ^= val[x];
            target.insert(x);
        } else {
            let mut remove: Vec<usize> = Vec::new();
            for &x in target.iter() {
                ans ^= val[x];
                val[x] -= 1;
                ans ^= val[x];
                if val[x] == 0 {
                    remove.push(x);
                }
            }
            for x in &remove {
                target.remove(x);
            }
        }
        println!("{}", ans);
    }
}