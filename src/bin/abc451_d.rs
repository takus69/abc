use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
    }

    let mut i = 1;
    let mut bs: Vec<usize> = vec![i];
    while i < 1_000_000_000 {
        i <<= 1;
        bs.push(i);
    }
    let mut ans: Vec<usize> = vec![0];
    dfs(0, &mut ans, &bs);

    ans.sort();
    ans.dedup();
    println!("{}", ans[n]);
}

fn dfs(i: usize, ans: &mut Vec<usize>, bs: &[usize]) {
    for &b in bs {
        let j = format!("{}{}", i, b).parse().unwrap();
        if j < 1_000_000_000 {
            ans.push(j);
            dfs(j, ans, bs);
        } else {
            break;
        }
    }
}