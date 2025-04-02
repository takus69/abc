use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 8],
    }
    let mut cols: Vec<bool> = vec![false; 8];
    let mut rows: Vec<bool> = vec![false; 8];
    for i in 0..8 {
        for j in 0..8 {
            if s[i][j] == '#' {
                cols[i] = true;
                rows[j] = true;
            }
        }
    }
    let mut ans = 0;
    for c in cols.iter() {
        if !c {
            for r in rows.iter() {
                if !r {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}