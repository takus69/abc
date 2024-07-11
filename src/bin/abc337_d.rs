use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h],
    }

    let mut ans: i64 = i64::MAX;
    // 横方向
    for i in 0..h {
        let mut k_cnt = 0;
        let mut a_cnt = 0;
        for j in 0..w {
            match s[i][j] {
                'o' => {
                    k_cnt += 1;
                },
                '.' => {
                    k_cnt += 1;
                    a_cnt += 1;
                },
                _ => {
                    k_cnt = 0;
                    a_cnt = 0;
                },
            }
            if k_cnt > k {
                k_cnt -= 1;
                if s[i][j-k] == '.' {
                    a_cnt -= 1;
                }
            }
            if k_cnt == k {
                ans = ans.min(a_cnt);
            }
        }
    }

    // 縦方向
    for j in 0..w {
        let mut k_cnt = 0;
        let mut a_cnt = 0;
        for i in 0..h {
            match s[i][j] {
                'o' => {
                    k_cnt += 1;
                },
                '.' => {
                    k_cnt += 1;
                    a_cnt += 1;
                },
                _ => {
                    k_cnt = 0;
                    a_cnt = 0;
                },
            }
            if k_cnt > k {
                k_cnt -= 1;
                if s[i-k][j] == '.' {
                    a_cnt -= 1;
                }
            }
            if k_cnt == k {
                ans = ans.min(a_cnt);
            }
        }
    }

    if ans == i64::MAX {
        ans = -1;
    }
    println!("{}", ans);
}