use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut perms: Vec<Vec<u32>> = vec![vec![0]];
    for i in 1..n {
        let mut next = Vec::new();
        for perm in perms.iter() {
            let max_i = perm.iter().max().unwrap();
            for j in 0..=(max_i+1) {
                let mut cloned = perm.clone();
                cloned.push(j);
                next.push(cloned);
            }
        }
        perms = next;
    }
    // println!("{}", perms.len());

    let mut ans = HashSet::new();
    for perm in perms.iter() {
        let mut b = vec![0; n];
        for (i, &pi) in perm.iter().enumerate() {
            b[pi as usize] += a[i];
        }
        let mut tmp = 0;
        for bi in b.iter() {
            tmp ^= bi;
        }
        ans.insert(tmp);
    }

    println!("{}", ans.len());
}