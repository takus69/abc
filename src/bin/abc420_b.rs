use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }
    let mut ans = vec![0; n];
    for j in 0..m {
        let mut x = 0;
        let mut y = 0;
        for i in 0..n {
            let ci = s[i][j];
            if ci == '0' {
                x += 1;
            } else {
                y += 1;
            }
        }
        for i in 0..n {
            let ci = s[i][j];
            if x==0 || y==0 {
                ans[i] += 1;
            } else if x < y {
                if ci == '0' {
                    ans[i] += 1;
                }
            } else {
                if ci == '1' {
                    ans[i] += 1;
                }
            }
        }
    }
    let mut ans2: Vec<usize> = Vec::new();
    let &max_score = ans.iter().max().unwrap();
    for (i, &score) in ans.iter().enumerate() {
        if score == max_score {
            ans2.push(i+1);
        }
    }
    println!("{}", ans2.iter().join(" "));
}