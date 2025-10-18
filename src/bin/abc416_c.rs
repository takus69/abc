use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        mut s: [String; n],
    }
    s.sort();
    let mut all: Vec<String> = Vec::new();
    for i in 0..n.pow(k as u32) {
        let mut j = i;
        let mut perm: Vec<usize> = Vec::new();
        for _ in 0..k {
            perm.push(j%n);
            j /= n;
        }
        let mut ans: Vec<&str> = Vec::new();
        for &j in perm.iter() {
            ans.push(&s[j]);
        }
        // println!("i: {}, perm: {:?}, ans: {:?}", i, perm, ans);
        all.push(ans.iter().join(""));
    }
    all.sort();
    // println!("{:?}", all);
    println!("{}", all[x-1]);
}