use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let target: Vec<char> = "snuke".chars().collect();
    let mut ans: Vec<String> = Vec::new();
    // 横方向
    for i in 0..h {
        for j in 0..=(w-5) {
            let mut tmp = s[i][j..(j+5)].to_vec();
            if tmp == target {
                for k in j..(j+5) {
                    ans.push(format!("{} {}", i+1, k+1));
                }
            }
            tmp.reverse();
            if tmp == target {
                for k in (j..(j+5)).rev() {
                    ans.push(format!("{} {}", i+1, k+1));
                }
            }
        }
    }
    // 縦方向
    for i in 0..=(h-5) {
        for j in 0..w {
            let mut tmp: Vec<char> = Vec::new();
            for k in i..(i+5) {
                tmp.push(s[k][j]);
            }
            if tmp == target {
                for k in i..(i+5) {
                    ans.push(format!("{} {}", k+1, j+1));
                }
            }
            tmp.reverse();
            if tmp == target {
                for k in (i..(i+5)).rev() {
                    ans.push(format!("{} {}", k+1, j+1));
                }
            }
        }
    }
    // 斜め方向(y = -x)
    for i in 0..=(h-5) {
        for j in 0..=(w-5) {
            let mut tmp: Vec<char> = Vec::new();
            for k in 0..5 {
                tmp.push(s[i+k][j+k]);
            }
            if tmp == target {
                for k in 0..5 {
                    ans.push(format!("{} {}", i+k+1, j+k+1));
                }
            }
            tmp.reverse();
            if tmp == target {
                for k in (0..5).rev() {
                    ans.push(format!("{} {}", i+k+1, j+k+1));
                }
            }
        }
    }
    // 斜め方向(y = x)
    for i in 4..h {
        for j in 0..=(w-5) {
            let mut tmp: Vec<char> = Vec::new();
            for k in 0..5 {
                tmp.push(s[i-k][j+k]);
            }
            if tmp == target {
                for k in 0..5 {
                    ans.push(format!("{} {}", i-k+1, j+k+1));
                }
            }
            tmp.reverse();
            if tmp == target {
                for k in (0..5).rev() {
                    ans.push(format!("{} {}", i-k+1, j+k+1));
                }
            }
        }
    }
    for a in ans.iter() {
        println!("{}", a);
    }
}