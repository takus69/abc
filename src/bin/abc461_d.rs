use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut k: usize,
        s: [Chars; h],
    }
    let mut cnt: Vec<Vec<usize>> = vec![vec![0; w+1]; h+1];
    // 横方向累積和
    for i in 0..h {
        for j in 0..w {
            cnt[i+1][j+1] = cnt[i+1][j] + if s[i][j] == '1' { 1 } else { 0 };
        }
    }
    // 縦方向累積和
    for j in 0..w {
        for i in 0..h {
            cnt[i+1][j+1] += cnt[i][j+1];
        }
    }

    let mut ans = 0;
    for i1 in 1..=h {
        for i2 in i1..=h {
            // 尺取りで横方向でk以上の数
            let mut j1 = 1;
            let mut j2 = 1;
            let mut cnt2 = 0;
            while j1 <= w && j2 <= w{
                let c = rect_cnt(i1, j1, i2, j2, &cnt);
                if c < k {
                    j2 += 1;
                } else {
                    cnt2 += w-j2+1;
                    j1 += 1;
                    if j1 > j2 {
                        j2 = j1;
                    }
                }
            }
            // 尺取りで横方向でk超の数
            let mut j1 = 1;
            let mut j2 = 1;
            let mut cnt1 = 0;
            while j1 <= w && j2 <= w {
                let c = rect_cnt(i1, j1, i2, j2, &cnt);
                if c <= k {
                    j2 += 1;
                } else {
                    cnt1 += w-j2+1;
                    j1 += 1;
                    if j1 > j2 {
                        j2 = j1;
                    }
                }
            }
            // println!("i1: {}, i2: {}, cnt2: {}, cnt1: {}", i1, i2, cnt2, cnt1);
            ans += cnt2 - cnt1;
        }
    }
    println!("{}", ans);

    fn rect_cnt(i1: usize, j1: usize, i2: usize, j2: usize, cnt: &[Vec<usize>]) -> usize {
        cnt[i2][j2] + cnt[i1-1][j1-1] - cnt[i2][j1-1] - cnt[i1-1][j2]
    }
}
