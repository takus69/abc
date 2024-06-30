use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        t: usize,
        s: Chars,
        x: [i64; n],
    }

    let mut sorted_x = x.clone();
    let mut indexed_x: Vec<(i64, usize)> = sorted_x.iter().enumerate().map(|(i, &val)| (val, i)).collect();
    indexed_x.sort();
    let mut s2 = Vec::new();
    let mut x2 = Vec::new();
    for (_, i) in indexed_x.iter() {
        x2.push(x[*i]);
        s2.push(s[*i]);
    }
    let x = x2;
    let s = s2;

    let mut ans: i64 = 0;
    let mut cnt = 0;
    let mut i2 = 1;
    for i in 0..n {
        i2 = i2.max(i+1);
        // println!("before i: {}, i2: {}, cnt: {}, ans: {}", i, i2, cnt, ans);
        let xi = x[i];
        let si = s[i];
        if si == '1' {
            let mut tmp = i2;
            for j in i2..n {
                if x[j] - xi <= 2*t  as i64 {
                    tmp = j+1;
                    if s[j] == '0' {
                        cnt += 1;
                    }
                } else {
                    break;
                }
            }
            i2 = tmp;
            ans += cnt;
        } else {
            if cnt > 0 {
                cnt -= 1;
            }
        }
        // println!("after i: {}, i2: {}, cnt: {}, ans: {}", i, i2, cnt, ans);
    }
    println!("{}", ans);
}