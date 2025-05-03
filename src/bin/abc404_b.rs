use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
        t: [Chars; n],
    }
    // 4回回転を試す
    let mut ans = usize::MAX;
    for c in 0..4 {
        let mut cnt = c;
        // 違う個数をチェック
        for i in 0..n {
            for j in 0..n {
                if s[i][j] != t[i][j] {
                    cnt += 1;
                }
            }
        }
        ans = ans.min(cnt);

        // 回転の処理
        let mut s2: Vec<Vec<char>> = vec![vec!['.'; n]; n];
        for i in 0..n {
            for j in 0..n {
                s2[j][n-i-1] = s[i][j];
            }
        }
        s = s2;
    }
    println!("{}", ans);
}