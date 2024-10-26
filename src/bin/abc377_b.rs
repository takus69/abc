use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 8],
    }
    let mut rows: Vec<usize> = Vec::new();
    let mut cols: Vec<usize> = Vec::new();
    for i in 0..8 {
        for j in 0..8 {
            if s[i][j] == '#' {
                rows.push(i);
                cols.push(j);
            }
        }
    }
    let mut ans = 0;
    for i in 0..8 {
        for j in 0..8 {
            if !rows.contains(&i) && !cols.contains(&j) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}