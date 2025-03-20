use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [[usize; n]; n],
        b: [[usize; n]; n],
    }
    for _ in 0..4 {
        let mut a2: Vec<Vec<usize>> = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                a2[i][j] = a[n-1-j][i];
            }
        }
        let mut flg = true;
        for i in 0..n {
            for j in 0..n {
                if a2[i][j] == 1 && b[i][j] == 0 {
                    flg = false;
                }
            }
        }
        if flg {
            println!("Yes");
            return;
        }
        a = a2;
    }
    println!("No");
}