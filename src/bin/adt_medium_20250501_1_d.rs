use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [[usize; n]; n],
        b: [[usize; n]; n],
    }
    for _ in 0..4 {
        let mut a2 = a.clone();
        // 回転
        for i in 0..n {
            for j in 0..n {
                a2[i][j] = a[n-j-1][i];
            }
        }
        a = a2;
        let mut ans = "Yes";
        for i in 0..n {
            for j in 0..n {
                if a[i][j] == 1 && b[i][j] == 0 {
                    ans = "No";
                }
            }
        }
        if ans == "Yes" {
            println!("Yes");
            return;
        }
    }
    println!("No");
}