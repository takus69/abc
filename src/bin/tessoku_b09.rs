use proconio::input;

fn main() {
    input! {
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    }
    let mut prefix: Vec<Vec<isize>> = vec![vec![0; 1503]; 1503];
    for &(a, b, c, d) in &abcd {
        prefix[a+1][b+1] += 1;
        prefix[c+1][b+1] -= 1;
        prefix[a+1][d+1] -= 1;
        prefix[c+1][d+1] += 1;
    }
    for i in 1..=1501 {
        for j in 1..=1501 {
            prefix[i][j] += prefix[i-1][j] + prefix[i][j-1] - prefix[i-1][j-1];
        }
    }
    let mut ans = 0;
    for i in 1..=1501 {
        for j in 1..=1501 {
            if prefix[i][j] > 0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}