use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let mut black: Vec<Vec<usize>> = vec![vec![0; n+1]; n+1];
    for j in 0..n {
        let mut cnt = 0;
        for i in 0..n {
            if s[i][j] == '#' {
                cnt += 1;
            }
            black[i+1][j+1] = cnt;
        }
    }
    let mut ans: Vec<Vec<usize>> = vec![vec![0; n+1]; n+1];
    for i in 0..=n {
        let b = n-i-(black[n][n]-black[i][n]);
        let w = black[i][n];
        ans[i][n] = b+w;
    }
    for j in (1..n).rev() {
        let mut cnt = usize::MAX;
        for i in 0..=n {
            cnt = cnt.min(ans[i][j+1]);
            let b = n-i-(black[n][j]-black[i][j]);
            let w = black[i][j];
            ans[i][j] = cnt+b+w;
        }
    }
    let mut a = usize::MAX;
    for i in 0..=n {
        a = a.min(ans[i][1]);
    }
    println!("{}", a);
}