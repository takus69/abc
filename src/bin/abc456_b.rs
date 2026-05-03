use proconio::input;

fn main() {
    input! {
        a: [[usize; 6]; 3],
    }
    let mut cnt: Vec<Vec<usize>> = vec![vec![0; 7]; 3];
    for i in 0..3 {
        for j in 0..6 {
            cnt[i][a[i][j]] += 1;
        }
    }
    let mut ans = 0.0;
    for p in [[4, 5, 6], [4, 6, 5], [5, 4, 6], [5, 6, 4], [6, 4, 5], [6, 5, 4]] {
        let mut tmp = 1.0;
        for i in 0..3 {
            tmp *= cnt[i][p[i]] as f64 / 6.0;
        }
        ans += tmp
    }
    println!("{}", ans);
}