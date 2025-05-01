use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }
    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            for (di, dj) in [(-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1)] {
                let mut tmp: Vec<char> = Vec::new();
                for k in 0..n {
                    let mut i2 = (n as i32 + i as i32 + di*k as i32) as usize;
                    i2 %= n;
                    let mut j2 = (n as i32 + j as i32 + dj*k as i32) as usize;
                    j2 %= n;
                    tmp.push(a[i2][j2]);
                }
                let tmp: usize = tmp.iter().join("").parse().unwrap();
                ans = ans.max(tmp);
            }
        }
    }
    
    println!("{}", ans);
}