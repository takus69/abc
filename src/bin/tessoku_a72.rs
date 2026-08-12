use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        c: [Chars; h],
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '#' {
                ans += 1;
            }
        }
    }
    let base_ans = ans;
    // hをビット全探索
    for b in 0..(1 << h) {
        let mut c2 = c.clone();
        let mut cnt= 0;
        let mut tmp_ans = base_ans;
        for i in 0..h {
            if (b >> i) & 1 == 1 {
                cnt += 1;
                for j in 0..w {
                    if c2[i][j] == '.' {
                        c2[i][j] = '#';
                        tmp_ans += 1;
                    }
                }
            }
        }
        if cnt > k { continue; }
        // wを残りが多い行から塗る
        let mut left: Vec<usize> = Vec::new();
        for j in 0..w {
            let mut tmp = 0;
            for i in 0..h {
                if c2[i][j] == '.' {
                    tmp += 1;
                }
            }
            left.push(tmp);
        }
        left.sort();
        while let Some(tmp) = left.pop() {
            if k - cnt == 0 { break; }
            cnt += 1;
            tmp_ans += tmp;
        }
        ans = ans.max(tmp_ans);
    }
    println!("{}", ans);
}