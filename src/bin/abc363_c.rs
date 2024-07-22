use proconio::{input, marker::Chars};
use std::collections::{HashSet, HashMap};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut s: Chars,
    }
    // let mut set: HashSet<Vec<char>> = HashSet::new();
    let mut set: Vec<Vec<char>> = Vec::new();
    for perm in s.into_iter().permutations(n).unique() {
        // set.insert(perm);
        set.push(perm);
    }
    let mut ans = set.len();
    for si in set.iter() {
        let mut ng = false;
        let mut si2 = si.clone();
        si2.reverse();
        for i in 0..=(n-k) {
            if si[i..(i+k)] == si2[(n-i-k)..(n-i)] {
                ng = true;
                break;
            }
            /*
            let mut ng2 = true;
            for j in 0..k {
                if si[i+j] != si[i+k-1-j] {
                    ng2 = false;
                    break;
                }
            }
            if ng2 {
                ng = true;
                break;
            }*/
        }
        if ng { ans -= 1; }
    }
    println!("{}", ans);
}
 