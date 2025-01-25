use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let mut hs = vec![];
    let mut ws = vec![];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                hs.push(i);
                ws.push(j);
            }
        }
    }
    let top = *hs.iter().min().unwrap();
    let bottom = *hs.iter().max().unwrap();
    let left = *ws.iter().min().unwrap();
    let right = *ws.iter().max().unwrap();
    let mut ans = "Yes";
    for i in top..=bottom {
        for j in left..=right {
            if s[i][j] == '.' {
                ans = "No";
                break;
            }
        }
    }
    println!("{}", ans);
}