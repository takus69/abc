use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
        x: [[usize; m]; n],
    }
    let mut sum_x = vec![0; m];
    for i in 0..n {
        for j in 0..m {
            sum_x[j] += x[i][j];
        }
    }
    let mut ans = "Yes";
    for i in 0..m {
        if a[i] > sum_x[i] {
            ans = "No";
            break;
        }
    }
    println!("{}", ans);
}