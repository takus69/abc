use proconio::input;
use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        p: [isize; n],
    }
    let mut cnt: Vec<usize> = vec![0; n+1];
    for &pi in p.iter() {
        if pi == -1 { continue; }
        cnt[pi as usize] += 1;
    }
    let mut set: HashSet<usize> = HashSet::new();
    for (i, &c) in cnt.iter().enumerate() {
        if c > 1 {
            println!("No");
            return;
        } else if c == 1 {
            set.insert(i);
        }
    }
    println!("Yes");
    let mut ii = 1;
    let mut ans: Vec<usize> = Vec::new();
    for &pi in p.iter() {
        let pi = pi as usize;
        if set.contains(&pi) {
            ans.push(pi);
        } else {
            while set.contains(&ii) {
                ii += 1;
            }
            ans.push(ii);
            ii += 1;
        }
    }
    println!("{}", ans.iter().join(" "));
}