use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [[usize; 10]; 10],
        a: [[i32; w]; h],
    }
    let mut mp = vec![[0; 10]; 10];
    for i in 0..10 {
        for j in 0..10 {
            mp[i][j] = c[i][j];
        }
    }
    for k in 0..10 {
        for i in 0..10 {
            for j in 0..10 {
                mp[i][j] = mp[i][j].min(mp[i][k] + mp[k][j]);
            }
        }
    }
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            let aij = a[i][j];
            if aij < 0 { continue; }
            ans += mp[aij as usize][1];
        }
    }

    println!("{}", ans);
}