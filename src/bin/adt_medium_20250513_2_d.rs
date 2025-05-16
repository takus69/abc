use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
    }
    let mut ans: HashSet<usize> = HashSet::new();
    for a in 1..400 {
        for b in 1..400 {
            let ss = 4*a*b + 3*a + 3*b;
            for (i, &si) in s.iter().enumerate() {
                if si == ss {
                    ans.insert(i);
                }
            }
        }
    }
    println!("{}", n-ans.len());
}